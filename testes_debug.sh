#!/usr/bin/env bash
set -uo pipefail

OUT="${OUT:-testes_debug.txt}"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
TMP_DIR="$(mktemp -d)"
APP_BIN="${APP_BIN:-}"

section() {
    printf '\n\n===== %s =====\n' "$*"
}

run() {
    printf '\n$ %s\n' "$*"
    "$@" 2>&1
    printf '[exit %s]\n' "$?"
}

restore_console() {
    if command -v python3 >/dev/null 2>&1; then
        python3 - >/dev/null 2>&1 <<'PY' || true
import fcntl
import os

try:
    fd = os.open("/dev/tty", os.O_RDWR | os.O_NOCTTY)
    try:
        try:
            fcntl.ioctl(fd, 0x4B3A, 0)  # KDSETMODE, KD_TEXT
        except OSError:
            pass
        os.write(fd, b"\x1b[0m\x1b[?25h")
    finally:
        os.close(fd)
except OSError:
    pass
PY
    else
        printf '\033[0m\033[?25h' >/dev/tty 2>/dev/null || true
    fi
}

on_signal() {
    echo "Interrompido por sinal."
    exit 130
}

cleanup() {
    local status=$?
    trap - EXIT
    restore_console
    rm -rf "$TMP_DIR"
    exit "$status"
}

trap on_signal INT TERM HUP
trap cleanup EXIT

printf 'Coletando diagnóstico em %s\n' "$OUT" >/dev/tty 2>/dev/null || true
exec >"$OUT" 2>&1

find_app() {
    if [ -n "$APP_BIN" ] && [ -x "$APP_BIN" ]; then
        printf '%s\n' "$APP_BIN"
        return 0
    fi

    if command -v velha-classica >/dev/null 2>&1; then
        command -v velha-classica
        return 0
    fi

    for candidate in \
        "$SCRIPT_DIR/velha-classica" \
        "$PWD/velha-classica" \
        "$SCRIPT_DIR/target/release/velha-classica" \
        "$PWD/target/release/velha-classica"
    do
        if [ -x "$candidate" ]; then
            printf '%s\n' "$candidate"
            return 0
        fi
    done

    return 1
}

blank_state() {
    printf '\n--- blank/DPMS atual ---\n'
    for f in /sys/class/graphics/fb*/blank; do
        [ -r "$f" ] && printf '%s: %s\n' "$f" "$(cat "$f")"
    done 2>/dev/null || true
}

run_app_test() {
    local label="$1"
    local app_out="$TMP_DIR/app-$RANDOM.out"
    local strace_log="$TMP_DIR/strace-$RANDOM.log"
    local rc

    section "$label — app real sob strace/telemetria de ioctl"
    blank_state

    if [ -z "$APP" ]; then
        printf 'app não encontrado; teste ignorado\n'
        return 0
    fi

    if command -v strace >/dev/null 2>&1 && strace -o /dev/null true >/dev/null 2>&1; then
        printf '\nExecutando por 10 s: strace + timeout ...\n'
        if command -v timeout >/dev/null 2>&1; then
            strace -f -tt -e trace=ioctl,openat,mmap,munmap,write -o "$strace_log" \
                timeout --signal=TERM --kill-after=5 10 "$APP" --backend fb \
                >"$app_out" 2>&1
            rc=$?
        else
            strace -f -tt -e trace=ioctl,openat,mmap,munmap,write -o "$strace_log" \
                "$APP" --backend fb >"$app_out" 2>&1 &
            local strace_pid=$!
            sleep 10
            kill -TERM "$strace_pid" 2>/dev/null || true
            wait "$strace_pid"
            rc=$?
        fi
        printf 'exit=%s (124 = timeout esperado)\n' "$rc"
        printf '\n--- stdout/stderr do app ---\n'
        cat "$app_out"
        printf '\n--- strace filtrado (fb/tty/ioctls/mmap) ---\n'
        grep -nE 'fb0|/dev/tty|ioctl|mmap|munmap|KDSET|KDGET|FBIOPAN|FBIOBLANK|FBIOGET|FBIOPUT|0x4b3|0x460|0x461' "$strace_log" | tail -400 || true
        printf '\n--- fim do strace filtrado ---\n'
        printf 'linhas totais no strace: %s\n' "$(wc -l < "$strace_log")"
    else
        printf 'strace ausente ou sem permissão; executando só o app com timeout\n'
        if command -v timeout >/dev/null 2>&1; then
            timeout --signal=TERM --kill-after=5 10 "$APP" --backend fb >"$app_out" 2>&1
            rc=$?
        else
            "$APP" --backend fb >"$app_out" 2>&1 &
            local app_pid=$!
            sleep 10
            kill -TERM "$app_pid" 2>/dev/null || true
            wait "$app_pid"
            rc=$?
        fi
        printf 'exit=%s (124 = timeout esperado)\n' "$rc"
        printf '\n--- stdout/stderr do app ---\n'
        cat "$app_out"
    fi

    printf '\n--- blank/DPMS depois do app ---\n'
    blank_state
    printf '\n--- processos velha-classica restantes ---\n'
    pgrep -a velha-classica 2>/dev/null || echo "nenhum"
    restore_console
}

run_probe_test() {
    section "TESTE 2 — escrita direta no framebuffer visível (desbloqueia o blank)"
    printf 'Este teste entra em KD_GRAPHICS, tenta desbloquear o blank, pinta faixas RGB na memória visível e volta para KD_TEXT.\n'
    if ! command -v python3 >/dev/null 2>&1; then
        printf 'python3 ausente; teste ignorado\n'
        return 0
    fi

    python3 - <<'PY'
import ctypes
import fcntl
import mmap
import os
import struct
import time

FBIOGET_VSCREENINFO = 0x4600
FBIOGET_FSCREENINFO = 0x4602
FBIOPAN_DISPLAY = 0x4606
FBIOBLANK = 0x4611
FB_BLANK_UNBLANK = 0
KDGETMODE = 0x4B3B
KDSETMODE = 0x4B3A
KD_TEXT = 0
KD_GRAPHICS = 1


class Bitfield(ctypes.Structure):
    _fields_ = [
        ("offset", ctypes.c_uint32),
        ("length", ctypes.c_uint32),
        ("msb_right", ctypes.c_uint32),
    ]


class Var(ctypes.Structure):
    _fields_ = [
        ("xres", ctypes.c_uint32),
        ("yres", ctypes.c_uint32),
        ("xres_virtual", ctypes.c_uint32),
        ("yres_virtual", ctypes.c_uint32),
        ("xoffset", ctypes.c_uint32),
        ("yoffset", ctypes.c_uint32),
        ("bits_per_pixel", ctypes.c_uint32),
        ("grayscale", ctypes.c_uint32),
        ("red", Bitfield),
        ("green", Bitfield),
        ("blue", Bitfield),
        ("transp", Bitfield),
        ("nonstd", ctypes.c_uint32),
        ("activate", ctypes.c_uint32),
        ("height", ctypes.c_uint32),
        ("width", ctypes.c_uint32),
        ("accel_flags", ctypes.c_uint32),
        ("pixclock", ctypes.c_uint32),
        ("left_margin", ctypes.c_uint32),
        ("right_margin", ctypes.c_uint32),
        ("upper_margin", ctypes.c_uint32),
        ("lower_margin", ctypes.c_uint32),
        ("hsync_len", ctypes.c_uint32),
        ("vsync_len", ctypes.c_uint32),
        ("sync", ctypes.c_uint32),
        ("vmode", ctypes.c_uint32),
        ("rotate", ctypes.c_uint32),
        ("colorspace", ctypes.c_uint32),
        ("reserved", ctypes.c_uint32 * 4),
    ]


class Fix(ctypes.Structure):
    _fields_ = [
        ("id", ctypes.c_char * 16),
        ("smem_start", ctypes.c_ulong),
        ("smem_len", ctypes.c_uint32),
        ("fb_type", ctypes.c_uint32),
        ("type_aux", ctypes.c_uint32),
        ("visual", ctypes.c_uint32),
        ("xpanstep", ctypes.c_uint16),
        ("ypanstep", ctypes.c_uint16),
        ("ywrapstep", ctypes.c_uint16),
        ("line_length", ctypes.c_uint32),
        ("mmio_start", ctypes.c_ulong),
        ("mmio_len", ctypes.c_uint32),
        ("accel", ctypes.c_uint32),
        ("capabilities", ctypes.c_uint16),
        ("reserved", ctypes.c_uint16 * 2),
    ]


def get_var(fd):
    buf = bytearray(ctypes.sizeof(Var))
    fcntl.ioctl(fd, FBIOGET_VSCREENINFO, buf, True)
    return Var.from_buffer(buf), buf


def get_fix(fd):
    buf = bytearray(ctypes.sizeof(Fix))
    fcntl.ioctl(fd, FBIOGET_FSCREENINFO, buf, True)
    return Fix.from_buffer(buf), buf


def channel(value, offset, length):
    if length == 0:
        return 0
    maximum = (1 << length) - 1
    return ((value * maximum + 127) // 255 & maximum) << offset


def pack(var, r, g, b):
    return (
        channel(r, var.red.offset, var.red.length)
        | channel(g, var.green.offset, var.green.length)
        | channel(b, var.blue.offset, var.blue.length)
        | channel(255, var.transp.offset, var.transp.length)
    )


fd = None
ttyfd = None
mapping = None
previous_mode = KD_TEXT
original_yoffset = 0
try:
    fd = os.open("/dev/fb0", os.O_RDWR | os.O_CLOEXEC)
    var, var_buf = get_var(fd)
    fix, _ = get_fix(fd)
    print(f"var_size={ctypes.sizeof(Var)} fix_size={ctypes.sizeof(Fix)}")
    print(
        "var: "
        f"xres={var.xres} yres={var.yres} "
        f"xres_virtual={var.xres_virtual} yres_virtual={var.yres_virtual} "
        f"xoffset={var.xoffset} yoffset={var.yoffset} bpp={var.bits_per_pixel}"
    )
    print(
        "fix: "
        f"line_length={fix.line_length} smem_len={fix.smem_len} "
        f"xpanstep={fix.xpanstep} ypanstep={fix.ypanstep} "
        f"ywrapstep={fix.ywrapstep} fb_type={fix.fb_type}"
    )
    print(
        "bitfields: "
        f"R@{var.red.offset}/{var.red.length} "
        f"G@{var.green.offset}/{var.green.length} "
        f"B@{var.blue.offset}/{var.blue.length} "
        f"A@{var.transp.offset}/{var.transp.length}"
    )
    original_yoffset = var.yoffset

    ttyfd = os.open("/dev/tty", os.O_RDWR | os.O_NOCTTY)
    mode_buf = bytearray(4)
    fcntl.ioctl(ttyfd, KDGETMODE, mode_buf, True)
    previous_mode = struct.unpack("i", mode_buf)[0]
    print(f"KDGETMODE antes: {previous_mode}")

    try:
        fcntl.ioctl(fd, FBIOBLANK, FB_BLANK_UNBLANK)
        print("FBIOBLANK(FB_BLANK_UNBLANK): ok")
    except OSError as error:
        print(f"FBIOBLANK(FB_BLANK_UNBLANK): erro {error.errno}: {error}")

    fcntl.ioctl(ttyfd, KDSETMODE, KD_GRAPHICS)
    print("KDSETMODE(KD_GRAPHICS): ok")
    os.write(ttyfd, b"\x1b[?25l\x1b[2J\x1b[H")

    bytes_per_pixel = var.bits_per_pixel // 8
    if bytes_per_pixel not in (2, 3, 4):
        raise RuntimeError(f"bpp não suportado pelo teste: {var.bits_per_pixel}")
    row_bytes = var.xres * bytes_per_pixel
    stride = fix.line_length
    if stride < row_bytes:
        raise RuntimeError(f"line_length {stride} < row_bytes {row_bytes}")

    map_len = stride * (var.yoffset + var.yres)
    if fix.smem_len and map_len > fix.smem_len:
        raise RuntimeError(f"map_len {map_len} > smem_len {fix.smem_len}")
    if map_len == 0:
        raise RuntimeError("map_len zero")

    mapping = mmap.mmap(
        fd,
        map_len,
        flags=mmap.MAP_SHARED,
        prot=mmap.PROT_READ | mmap.PROT_WRITE,
    )

    bands = [(255, 0, 0), (0, 255, 0), (0, 0, 255), (255, 255, 255)]
    for y in range(var.yres):
        band = min(3, (y * 4) // var.yres)
        r, g, b = bands[band]
        pixel = pack(var, r, g, b).to_bytes(bytes_per_pixel, "little")
        row = pixel * var.xres
        row += b"\x00" * (stride - row_bytes)
        start = (var.yoffset + y) * stride
        mapping[start : start + stride] = row
    try:
        mapping.flush()
    except OSError as error:
        print(f"aviso: mmap.flush() falhou (não fatal): {error}")
    marker = (var.yoffset * stride, bytes(mapping[var.yoffset * stride : var.yoffset * stride + 16]))
    print(f"escrita visível: offset={marker[0]} primeiros bytes={marker[1].hex()}")
    print("aguardando 4 segundos com as faixas RGB na tela...")
    time.sleep(4)

    if (
        fix.ypanstep > 0
        and var.yres_virtual >= var.yres * 2
        and var.yres % fix.ypanstep == 0
        and fix.smem_len >= stride * 2 * var.yres
    ):
        target = 0 if var.yoffset != 0 else var.yres
        start = target * stride
        for y in range(var.yres):
            band = min(3, (y * 4) // var.yres)
            r, g, b = bands[3 - band]
            pixel = pack(var, r, g, b).to_bytes(bytes_per_pixel, "little")
            row = pixel * var.xres
            row += b"\x00" * (stride - row_bytes)
            mapping[start + y * stride : start + y * stride + stride] = row
        var.yoffset = target
        try:
            fcntl.ioctl(fd, FBIOPAN_DISPLAY, var_buf, True)
            print(f"FBIOPAN_DISPLAY(target={target}): ok")
            time.sleep(3)
            var.yoffset = original_yoffset
            fcntl.ioctl(fd, FBIOPAN_DISPLAY, var_buf, True)
            print(f"FBIOPAN_DISPLAY(target={original_yoffset}): restaurado")
        except OSError as error:
            print(f"FBIOPAN_DISPLAY(target={target}): erro {error.errno}: {error}")
            var.yoffset = original_yoffset
    else:
        print("pan não testado: ypanstep/yres_virtual/smem_len não habilitam double buffer")
except Exception as error:
    print(f"TESTE 2 falhou: {type(error).__name__}: {error}")
finally:
    try:
        if ttyfd is not None:
            fcntl.ioctl(ttyfd, KDSETMODE, KD_TEXT)
            os.write(ttyfd, b"\x1b[0m\x1b[?25h\x1b[2J\x1b[H")
    except OSError as error:
        print(f"restauração KD_TEXT falhou: {error}")
    try:
        if mapping is not None:
            mapping.close()
    except Exception:
        pass
    if fd is not None:
        os.close(fd)
    if ttyfd is not None:
        os.close(ttyfd)
PY
}

APP="$(find_app || true)"

section "Cabeçalho"
printf 'data: %s\n' "$(date -Is 2>/dev/null || date)"
printf 'script: %s\n' "$SCRIPT_DIR/$(basename "$0")"
printf 'saida: %s\n' "$OUT"
printf 'hostname: %s\n' "$(hostname 2>/dev/null)"
printf 'kernel: %s\n' "$(uname -a)"
printf 'app: %s\n' "${APP:-NÃO ENCONTRADO}"

section "Sistema"
run cat /etc/os-release
run uname -m
run cat /proc/cmdline
run cat /proc/version

section "Usuário, sessão e terminal"
run id
run sh -c 'id -nG'
run sh -c 'printf "tty=%s\nTERM=%s\nXDG_SESSION_TYPE=%s\nDISPLAY=%s\nWAYLAND_DISPLAY=%s\n" "$(tty 2>/dev/null)" "${TERM:-}" "${XDG_SESSION_TYPE:-}" "${DISPLAY:-}" "${WAYLAND_DISPLAY:-}"'
run sh -c 'cat /sys/class/tty/tty0/active 2>/dev/null || true'
run sh -c 'command -v fgconsole >/dev/null && fgconsole || echo "fgconsole ausente"'
run sh -c 'loginctl 2>/dev/null | head -20 || true'

section "Permissões do framebuffer"
run ls -l /dev/fb0 /dev/fb1 /dev/fb2 2>/dev/null
run sh -c 'stat /dev/fb0 2>/dev/null || true'
run sh -c 'command -v getfacl >/dev/null && getfacl /dev/fb0 2>/dev/null || echo "getfacl ausente"'
run sh -c 'command -v udevadm >/dev/null && udevadm info --query=all --name=/dev/fb0 2>/dev/null || echo "udevadm ausente"'

section "Framebuffers do kernel"
run cat /proc/fb
run sh -c 'ls -l /dev/fb* 2>/dev/null || true'
run sh -c 'for d in /sys/class/graphics/fb*; do [ -d "$d" ] || continue; echo "--- $d"; for f in name virtual_size bits_per_pixel stride blank modes pan rotate; do [ -r "$d/$f" ] && echo "$f: $(cat "$d/$f")"; done; readlink -f "$d/device/driver" 2>/dev/null; done'

section "fbset"
run sh -c 'command -v fbset >/dev/null && fbset -i || echo "fbset ausente"'
run sh -c 'command -v fbset >/dev/null && for d in /dev/fb*; do echo "--- $d"; fbset -fb "$d" -i 2>&1; done || true'

section "Console virtual, fbcon e KD_GRAPHICS"
run sh -c 'ls -l /sys/class/graphics/fbcon 2>/dev/null || true'
run sh -c 'for f in /sys/class/vtconsole/vtcon*/name; do [ -r "$f" ] && echo "$f: $(cat "$f")"; done'
run sh -c 'cat /sys/class/graphics/fbcon/name 2>/dev/null || true'
run sh -c 'cat /sys/class/tty/console/active 2>/dev/null || true'
run sh -c 'cat /sys/class/tty/tty0/active 2>/dev/null || true'
run sh -c 'ls -l /dev/tty /dev/tty0 2>/dev/null || true'

section "DRM, GPU e módulos"
run sh -c 'ls -l /dev/dri 2>/dev/null || true'
run sh -c 'for d in /sys/class/drm/card*/status; do [ -r "$d" ] && echo "$d: $(cat "$d")"; done 2>/dev/null || true'
run sh -c 'for d in /sys/class/drm/card*/enabled; do [ -r "$d" ] && echo "$d: $(cat "$d")"; done 2>/dev/null || true'
run sh -c 'for d in /sys/class/drm/card*; do [ -e "$d/device/driver" ] && echo "$d -> $(readlink -f "$d/device/driver")"; done 2>/dev/null || true'
run sh -c 'lsmod 2>/dev/null | grep -Ei "fb|drm|amdgpu|i915|nouveau|radeon|simpledrm|efifb|vesafb|virtio" || true'
run sh -c 'command -v lspci >/dev/null && lspci -nnk | grep -A4 -Ei "vga|display|3d" || echo "lspci ausente"'

section "Estado de blank/DPMS"
run sh -c 'for f in /sys/class/graphics/fb*/blank; do [ -r "$f" ] && echo "$f: $(cat "$f")"; done 2>/dev/null || true'
run sh -c 'command -v dmesg >/dev/null && dmesg 2>/dev/null | grep -Ei "fb0|fbcon|drm|blank|dpms|amdgpu|i915|simpledrm|efifb" | tail -80 || true'

section "Versão e dependências do app"
if [ -n "$APP" ]; then
    run "$APP" --version
    run file "$APP"
    run sh -c "ldd '$APP' 2>/dev/null || true"
else
    printf 'binário não encontrado (procurei velha-classica no PATH e em ./target/release)\n'
fi
run sh -c 'command -v strace || true'
run sh -c 'command -v timeout || true'
run sh -c 'command -v python3 || true'
run python3 --version

run_app_test "TESTE 1"
run_probe_test
run_app_test "TESTE 3 — após o teste de desbloqueio"

section "Estado depois dos testes"
run sh -c 'for f in /sys/class/graphics/fb*/blank; do [ -r "$f" ] && echo "$f: $(cat "$f")"; done 2>/dev/null || true'
run sh -c 'cat /sys/class/tty/tty0/active 2>/dev/null || true'
run sh -c 'command -v fbset >/dev/null && fbset -i || true'
run sh -c 'command -v dmesg >/dev/null && dmesg 2>/dev/null | tail -60 || true'

section "Resumo do arquivo"
printf 'arquivo: %s\n' "$OUT"
printf 'tamanho: %s bytes\n' "$(wc -c < "$OUT" 2>/dev/null || echo "?")"
printf 'linhas: %s\n' "$(wc -l < "$OUT" 2>/dev/null || echo "?")"

restore_console
printf '\nDiagnóstico concluído. Envie o arquivo: %s\n' "$OUT" >/dev/tty 2>/dev/null || true
