# Avro phonetic for Linux in IBus
Avro phonetic implementation for Linux in IBus.

## Installation


<a href="https://repology.org/project/ibus-avro/versions">
    <img src="https://repology.org/badge/vertical-allrepos/ibus-avro.svg" alt="Packaging status" align="right">
</a>

Ubuntu/Debian/Linux Mint
---

On Ubuntu, and on Debian's *testing* and *unstable* releases, Avro phonetic
is distributed through the `ibus-avro` package. To install it, simply do:

	sudo apt install ibus-avro

Arch/Manjaro/EndeavourOS
---

Install [`ibus-avro-git`](https://aur.archlinux.org/packages/ibus-avro-git) from the AUR. Please see [AUR prerequisites](https://wiki.archlinux.org/title/Arch_User_Repository#Prerequisites) if you already have not.


Other Linux distros (install from source)
---

On other Linux distros you can install the dependencies and build/install
using the source code in this repository.

1. Open terminal/package manager and install following packages:

		git
		libibus-1.0-dev
		automake
		autoconf
		make
		gjs
		ibus

    __For e.g. Debian 10 "buster"__

    As root, do:

		apt install git libibus-1.0-dev automake autoconf make gjs ibus

    __For other linux distributions__

    You'll need all related build tools like `automake`, `autoconf` etc...
    and to run it you need `ibus` and `gjs`. Use the list of packages above
    as guidance, but please note that some packages may have other names.

2. Clone the project and enter its directory:

		git clone https://github.com/turjaun985123/ibus-avro-rust.git
		cd ibus-avro-rust

3. Download the `avro-phonetic-ibus-linux-x86_64` artifact from the latest successful [Rust core workflow](https://github.com/turjaun985123/ibus-avro-rust/actions/workflows/rust-core.yml), extract it, and place the executable at `rust/avro-phonetic/target/release/avro-phonetic-ibus` in the checkout. The package install now requires this CI-built Rust engine.

4. Generate the install files, configure, and install:

		aclocal && autoconf && automake --add-missing
		./configure --prefix=/usr
		sudo make install
		
    __For Fedora (36, Cinnamon DE), Installing from source, and Enabling IBus Avro__

		git clone https://github.com/turjaun985123/ibus-avro-rust.git
		cd ibus-avro-rust
		sudo dnf install automake # this installs aclocal, autoconf, and automake
		sudo dnf install ibus-devel ibus-libs  # to repair "missing ibus-1.0 error"
		# Download and stage the CI artifact as described above before installing.
		aclocal && autoconf && automake --add-missing
		./configure --prefix=/usr

		sudo make install
	
	Avro should be installed on your system. If you can't find anything, try logging out / restarting the computer. 
	Now one needs to add avro in his input method. Since the installation was done on Fedora 36 with Cinnamon DE, it maybe slightly different in other DE (ie, Gnome, KDE, etc).
	
	Press `windows key` -> `Input method selector`.
	Initially, `no input method` was chosen. Select `Use IBus`, and at the right of `Use IBus`, there is `Preference`. Clicking it opens
	`IBus preference`. Now `Input method` -> `Add` -> `Bangla` -> `Avro`. One may need to logout again to see the changes.

## Usage

**Older versions of GNOME, Cinnamon, and present version of other DEs**:

 1. Run __IBus__ (`Applications -> System Tools -> IBus`) from _Dash_
 2. Open __IBus__ `Preferences` from the top panel icon  
 3. Go to `Input method`
 4. `Select an input method -> Bengali -> Avro`
 5. Now Click `Add` button to add __Avro__ to the list
 6. Now restart __IBus__ from the top panel icon (`Right Click -> Restart`)
 7. Now Press `Ctrl+Space` to toggle between _English_ and _Avro_ (Bengali)
 8. Enjoy __Avro Phonetic!__

**Recent GNOME versions**: It requires additional steps to configure ibus-avro in Manjaro GNOME (and possibly other distros using a very recent GNOME version). See [this](https://github.com/sarim/ibus-avro/issues/202#issuecomment-1719779633) comment for more details.

**Recent Cinnamon versions**: See [this comment](https://github.com/sarim/ibus-avro/issues/226#issuecomment-3762641612).

## Rust port

The Rust engine now powers the regular `ibus-avro` IBus entry. The port covers the ordered transliteration patterns and context rules, dictionary matching, autocorrect, suffix candidates, ranking, learned candidate choices, per-engine key and selection state, GSettings options, and the IBus preferences property. The existing GJS preferences window remains available from the engine's setup property. Installation requires the `avro-phonetic-ibus-linux-x86_64` binary artifact from a successful [Rust core workflow](https://github.com/turjaun985123/ibus-avro-rust/actions/workflows/rust-core.yml), staged at `rust/avro-phonetic/target/release/avro-phonetic-ibus`. GitHub Actions owns Rust builds and validation; do not run Cargo or local Rust builds. A live IBus session check remains outstanding.

## Contributors
 
__IBus Engine__ by __Sarim Khan__ <sarim2005@gmail.com>

[__Avro JavaScript Phonetic Library__](https://github.com/torifat/jsAvroPhonetic) by [__Rifat Nabi__](https://github.com/torifat)

__Avro Phonetic Dictionary Search Library__ by [__Mehdi Hasan Khan__](https://github.com/omicronlab)

_Licensed under Mozilla Public License 2.0 ("MPL"), an open source/free software license._
