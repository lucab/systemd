/***
  This file is part of systemd.

  Copyright 2014 Lennart Poettering

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

#include <stdio.h>

_rust_fn_ void help(char *short_name);

void help(char *short_name) {
        printf("%s [OPTIONS...]\n\n"
               "Configures basic settings of the system.\n\n"
               "  -h --help                    Show this help\n"
               "     --version                 Show package version\n"
               "     --root=PATH               Operate on an alternate filesystem root\n"
               "     --locale=LOCALE           Set primary locale (LANG=)\n"
               "     --locale-messages=LOCALE  Set message locale (LC_MESSAGES=)\n"
               "     --timezone=TIMEZONE       Set timezone\n"
               "     --hostname=NAME           Set host name\n"
               "     --machine-ID=ID           Set machine ID\n"
               "     --root-password=PASSWORD  Set root password\n"
               "     --root-password-file=FILE Set root password from file\n"
               "     --prompt-locale           Prompt the user for locale settings\n"
               "     --prompt-timezone         Prompt the user for timezone\n"
               "     --prompt-hostname         Prompt the user for hostname\n"
               "     --prompt-root-password    Prompt the user for root password\n"
               "     --prompt                  Prompt for all of the above\n"
               "     --copy-locale             Copy locale from host\n"
               "     --copy-timezone           Copy timezone from host\n"
               "     --copy-root-password      Copy root password from host\n"
               "     --copy                    Copy locale, timezone, root password\n"
               "     --setup-machine-id        Generate a new random machine ID\n"
               , short_name);
}
