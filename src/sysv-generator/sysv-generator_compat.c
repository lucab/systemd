/***
    This file is part of systemd.

    Copyright 2014 Thomas H.P. Andersen
    Copyright 2010 Lennart Poettering
    Copyright 2011 Michal Schmidt

    systemd is free software; you can redistribute it and/or modify it
    under the terms of the GNU Lesser General Public License as published by
    the Free Software Foundation; either version 2.1 of the License, or
    (at your option) any later version.

    systemd is distributed in the hope that it will be useful, but
    WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
    Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with systemd; If not, see <http://www.gnu.org/licenses/>.
***/

#include <assert.h>
#include <errno.h>
#include <unistd.h>

#include "string-util.h"

_rust_fn_ int add_alias(const char *arg_dest, const char *service, const char *alias);

int add_alias(const char *arg_dest, const char *service, const char *alias) {
        const char *link;
        int r;

        assert(service);
        assert(alias);

        link = strjoina(arg_dest, "/", alias);

        r = symlink(service, link);
        if (r < 0) {
                if (errno == EEXIST)
                        return 0;

                return -errno;
        }

        return 1;
}
