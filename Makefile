# ************************************************************************************************************ #
#                                                                                                              #
#                                                      :::::::::  ::::::::   ::::::::   :::    ::: ::::::::::  #
#  Makefile                                           :+:    :+: :+:    :+: :+:    :+: :+:    :+: :+:          #
#                                                    +:+    +:+ +:+    +:+ +:+        +:+    +:+ +:+           #
#  By: se-yukun <yukun@doche.io>                    +#+    +:+ +#+    +:+ +#+        +#++:++#++ +#++:++#       #
#                                                  +#+    +#+ +#+    +#+ +#+        +#+    +#+ +#+             #
#  Created: 2023/08/16 21:18:12 by se-yukun       #+#    #+# #+#    #+# #+#    #+# #+#    #+# #+#              #
#  Updated: 2023/08/16 22:46:03 by se-yukun      #########  ########   ########  ###    ### ##########.io.     #
#                                                                                                              #
# ************************************************************************************************************ #

all: update build 

clean:
	rm -rf ./target/release

fclean:
	rm -rf ./target;
	rm -rf ./Cargo.lock;

re: fclean build

build:
	cargo build --release;

update:
	git pull;

upgrade: update build install

install:
	sudo install ./target/release/tardis /usr/local/bin;
	cp ./service/tardisd.service /etc/systemd/system/;

systemd:
	systemctl daemon-reload;
	systemctl enable --now tardisd;
