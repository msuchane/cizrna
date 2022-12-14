Name: cizrna
Summary: Generate an AsciiDoc release notes document from tracking tickets.
Version: 0.17.1
Release: 1%{?dist}
License: GPLv3+
URL: https://github.com/msuchane/cizrna
Group: Applications/Text
#Source0: https://static.crates.io/crates/%{name}/%{name}-%{version}.crate
Source0: https://github.com/msuchane/%{name}/archive/refs/tags/v%{version}.tar.gz

# This works fine with Fedora and RHEL, but breaks the SUSE build:
# ExclusiveArch: %{rust_arches}

BuildRequires: rust
BuildRequires: cargo
BuildRequires: openssl-devel

Requires: openssl-libs

%description
%{summary}

# Disable debugging packages. RPM looks for them even though none are created,
# and that breaks the build if you don't set this option.
%global debug_package %{nil}

%prep
# Unpack the sources.
%setup -q

%build
# Build the binary.
cargo build --release

%install
# Clean up previous artifacts.
rm -rf %{buildroot}
# Prepare the target directories.
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_mandir}/man1
# Install the binary into the chroot environment.
install -m 0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
# An alternative way to install the binary using cargo.
# cargo install --path . --root %{buildroot}/usr
# Install the man page into the chroot environment.
install -m 0644 %{name}.1 %{buildroot}%{_mandir}/man1/%{name}.1

%clean
rm -rf %{buildroot}

%files
# Pick documentation and license files from the source directory.
%doc README.md
# %doc CHANGELOG.md
%license LICENSE
%{_mandir}/man1/%{name}.1
# Pick the binary from the virtual, chroot system.
%{_bindir}/%{name}
