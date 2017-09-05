source = ['sysv-generator.c']
args = []

if rustless
        source += ['sysv-generator_compat.c']
else
        sysv_generator_rs = custom_target('sysv-generator_rs',
                                     input : ['Cargo.toml', 'sysv-generator.rs'],
                                     command : [cargo_helper, 'release', 'sysv-generator', '@INPUT0@'],
                                     output : ['libsysv_generator_rs.a'])
        source += [sysv_generator_rs]
        args += ['-lpthread']
endif

executable('systemd-sysv-generator',
           source,
           include_directories : includes,
           link_args : args,
           link_with : [libshared],
           install_rpath : rootlibexecdir,
           install : true,
           install_dir : systemgeneratordir)
