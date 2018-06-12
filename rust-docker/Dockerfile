FROM ubuntu:16.04
ENV DEBIAN_FRONTEND "noninteractive"

RUN apt-get update -y
RUN apt-get -y \
  -o Dpkg::Options::="--force-confdef" \
  -o Dpkg::Options::="--force-confold" dist-upgrade

# Install utilities(optional).
RUN apt-get install -y --no-install-recommends \
  dconf-tools \
  apt-transport-https software-properties-common ppa-purge apt-utils \
  ca-certificates git curl wget \
  tar zip unzip zlib1g-dev bzip2 libbz2-dev \
  openssl libssl-dev \
  zsh vim screen tree htop \
  net-tools lynx iftop traceroute \
  sudo

# Install gcc and clang
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN add-apt-repository "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-5.0 main"
RUN add-apt-repository ppa:ubuntu-toolchain-r/test
RUN apt-get update -y
RUN apt-get install -y --no-install-recommends \
  build-essential binutils cmake autoconf automake autogen pkg-config libtool \
  gcc-6 g++-6 gcc-7 g++-7 gdb \
  clang-5.0 lldb-5.0 lld-5.0
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-6 20
RUN update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-6 20

# neovim
RUN add-apt-repository ppa:neovim-ppa/unstable
RUN apt-get update
RUN apt-get install -y neovim
RUN apt-get install -y python3-dev python3-pip
RUN update-alternatives --install /usr/bin/vi vi /usr/bin/nvim 60
RUN update-alternatives --config vi
RUN update-alternatives --install /usr/bin/vim vim /usr/bin/nvim 60
RUN update-alternatives --config vim
RUN update-alternatives --install /usr/bin/editor editor /usr/bin/nvim 60
RUN update-alternatives --config editor

ENV LANGUAGE=C.UTF-8
ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8
ENV LC_CTYPE=C.UTF-8

RUN pip3 install -U pip
RUN pip3 install -U gdbgui

RUN apt-get install -y -f
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get dist-upgrade -y

RUN apt-get clean -y
RUN apt-get autoremove -y
RUN apt-get autoclean -y
RUN rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

ARG user_name=ubuntu
ARG user_id=1942
ARG group_name=ubuntu
ARG group_id=1942

RUN groupadd -g ${group_id} ${group_name}
RUN useradd -u ${user_id} -g ${group_id} -d /home/${user_name} --create-home --shell /usr/bin/zsh ${user_name}
RUN echo "${user_name} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
RUN chown -R ${user_name}:${group_name} /home/${user_name}
RUN chsh -s /usr/bin/zsh ${user_name}

USER ${user_name}
WORKDIR /home/${user_name}
ENV HOME /home/${user_name}

# rust
ENV RUST_VERSION stable
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${RUST_VERSION}
ENV PATH $PATH:$HOME/.cargo/bin
RUN rustup install stable
RUN rustup install beta
RUN rustup install nightly
RUN rustup component add rustfmt-preview
RUN rustup component add rls-preview rust-analysis rust-src

RUN cargo install racer
RUN cargo +nightly install --force clippy
RUN cargo install cargo-watch
RUN cargo install cargo-tree
RUN cargo install cargo-asm
RUN cargo install cargo-expand
RUN cargo install --git https://github.com/japaric/cargo-binutils
RUN cargo +nightly install cargo-src
RUN cargo install cargo-check

RUN bash -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
RUN echo 'shell "/usr/bin/zsh"' >> $HOME/.screenrc

RUN echo "\n\
mkdir -p $HOME/.zfunc\n\
rustup completions zsh > ~/.zfunc/_rustup\n\
fpath+=~/.zfunc\n\
" >> $HOME/.zshrc

# neovim + dein.vim + rust.vim + vim-racer
ENV XDG_CONFIG_HOME="$HOME/.config"
RUN mkdir -p $HOME/.config/nvim
RUN mkdir -p $HOME/.cache/dein
RUN touch $HOME/.config/nvim/init.vim
RUN wget -q -O - https://raw.githubusercontent.com/Shougo/dein.vim/master/bin/installer.sh | bash -s -- $HOME/.cache/dein
RUN echo "\n\
\n\
if &compatible\n\
  set nocompatible\n\
endif\n\
\n\
set runtimepath+=~/.cache/dein/repos/github.com/Shougo/dein.vim\n\
if dein#load_state('~/.cache/dein')\n\
  call dein#begin('~/.cache/dein')\n\
  \n\
  call dein#add('Shougo/dein.vim')\n\
  \n\
  call dein#add('rust-lang/rust.vim')\n\
  call dein#add('racer-rust/vim-racer')\n\
  \n\
  call dein#end()\n\
  call dein#save_state()\n\
endif\n\
\n\
let g:racer_cmd = expand('~/.cargo/bin/racer')\n\
let g:racer_experimental_completer = 1\n\
let g:racer_insert_paren = 1\n\
let g:rustfmt_command = expand('~/.cargo/bin/rustfmt')\n\
let g:rustfmt_autosave = 1\n\
\n\
filetype plugin indent on\n\
syntax enable\n\
\n\
if dein#check_install()\n\
  call dein#install()\n\
endif\n\
" >> $HOME/.config/nvim/init.vim

# git
RUN git config --global user.email "legokichi@gmail.com"
RUN git config --global user.name "legokichi"


