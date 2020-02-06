# SPDX-License-Identifier: LGPL-2.1+

cbindgen = find_program('cbindgen')
cargo = find_program('cargo')
# TODO(lucab): remove this, cargo can do SONAME directly.
patchelf = find_program('patchelf')

srcdir = meson.current_source_dir()

# TODO(lucab): remove this, cargo can emit depfile directly.
sources = ['src/lib.rs', 'src/sysv_generator/mod.rs']
header_name = 'rshared_ffi.h'
cbindgen_cfg = join_paths(srcdir, 'cbindgen.toml')
rshared_ffi_header = custom_target(
        header_name,
        build_by_default: true,
        build_always_stale: true,
        depends_files : sources,
        output : header_name,
        command : [cbindgen, '--quiet', '--config', cbindgen_cfg, srcdir],
        capture : true,
)

rshared_ffi_soname = 'librshared_ffi-@0@'.format(meson.project_version())
rshared_ffi_filename = rshared_ffi_soname + '.so'
cargo_wrapper = join_paths(meson.current_source_dir(), '../../../tools/meson-cargo-cdylib.sh')
librshared_ffi = custom_target(
        'rshared_ffi',
        build_by_default: true,
        build_always_stale: true,
        install: true,
        install_dir : rootlibexecdir,
        depends_files: rshared_ffi_header,
        output : rshared_ffi_filename,
        command : [cargo_wrapper, meson.build_root(), rshared_ffi_soname],
        capture : false,
)
