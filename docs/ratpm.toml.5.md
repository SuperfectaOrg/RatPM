Name:           ratpm
Version:        1.0.0
Release:        1%{?dist}
Summary:        RatOS Package Manager

License:        GPL-3.0-or-later
URL:            https://github.com/SuperfectaOrg/RatPM
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  libdnf5-devel
BuildRequires:  rpm-devel
BuildRequires:  gcc
BuildRequires:  pkgconfig

Requires:       rpm
Requires:       libdnf5
Requires:       systemd

%description
RatPM is the primary package management frontend for RatOS, a Fedora-based
Linux distribution. It provides a policy enforcement layer and user interface
on top of RPM and libdnf5.

%prep
%autosetup

%build
cargo build --release

%install
install -D -m 755 target/release/ratpm %{buildroot}%{_bindir}/ratpm
install -D -m 644 ratpm.toml.example %{buildroot}%{_sysconfdir}/ratpm/ratpm.toml
install -D -m 644 systemd/ratpm-refresh.service %{buildroot}%{_unitdir}/ratpm-refresh.service
install -D -m 644 systemd/ratpm-refresh.timer %{buildroot}%{_unitdir}/ratpm-refresh.timer
install -D -m 644 docs/ratpm.8 %{buildroot}%{_mandir}/man8/ratpm.8
install -D -m 644 docs/ratpm.toml.5 %{buildroot}%{_mandir}/man5/ratpm.toml.5

install -d -m 755 %{buildroot}%{_localstatedir}/cache/ratpm
install -d -m 755 %{buildroot}%{_localstatedir}/lib/ratpm

%files
%license LICENSE
%doc README.md
%{_bindir}/ratpm
%dir %{_sysconfdir}/ratpm
%config(noreplace) %{_sysconfdir}/ratpm/ratpm.toml
%{_unitdir}/ratpm-refresh.service
%{_unitdir}/ratpm-refresh.timer
%{_mandir}/man8/ratpm.8*
%{_mandir}/man5/ratpm.toml.5*
%dir %{_localstatedir}/cache/ratpm
%dir %{_localstatedir}/lib/ratpm

%post
%systemd_post ratpm-refresh.timer

%preun
%systemd_preun ratpm-refresh.timer

%postun
%systemd_postun_with_restart ratpm-refresh.timer

%changelog
* Mon Jan 26 2026 RatOS Team <team@superfecta.org> - 1.0.0-1
- Initial release
- Core package management functionality
- Repository management
- Transaction support
- Lock management
- Systemd timer integration
