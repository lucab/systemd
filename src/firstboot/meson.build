source = ['firstboot.c']
args = []

if rustless
        source += ['firstboot_compat.c']
else
        firstboot_rs = custom_target('firstboot_rs',
                                     input : ['Cargo.toml', 'firstboot.rs'],
                                     command : [cargo_helper, 'release', 'firstboot', '@INPUT0@'],
                                     output : ['libfirstboot_rs.a'])
        source += [firstboot_rs]
        args += ['-lpthread']
endif

executable('systemd-firstboot',
           source,
           include_directories : includes,
           link_args : args,
           link_with : [libshared],
           dependencies : [libcrypt],
           install_rpath : rootlibexecdir,
           install : true,
           install_dir : rootbindir)
