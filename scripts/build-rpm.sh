#!/bin/bash
set -e

VERSION=${1:-1.0.0}
RELEASE=${2:-1}

echo "Building RatPM RPM package..."
echo "Version: $VERSION"
echo "Release: $RELEASE"

if ! command -v rpmbuild &> /dev/null; then
    echo "Error: rpmbuild not found"
    echo "Install with: dnf install rpm-build"
    exit 1
fi

echo "Setting up RPM build environment..."
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

echo "Building release binary..."
cargo build --release

echo "Creating source tarball..."
TARBALL="ratpm-${VERSION}.tar.gz"
tar czf ~/rpmbuild/SOURCES/$TARBALL \
    --transform "s,^,ratpm-${VERSION}/," \
    --exclude='.git*' \
    --exclude='target' \
    --exclude='.cache' \
    --exclude='*.rpm' \
    .

echo "Copying spec file..."
cp packaging/ratpm.spec ~/rpmbuild/SPECS/

echo "Building RPM..."
rpmbuild -ba ~/rpmbuild/SPECS/ratpm.spec \
    --define "version ${VERSION}" \
    --define "release ${RELEASE}"

echo ""
echo "Build complete!"
echo ""
echo "RPM packages:"
ls -lh ~/rpmbuild/RPMS/x86_64/ratpm-*.rpm
echo ""
echo "SRPM package:"
ls -lh ~/rpmbuild/SRPMS/ratpm-*.src.rpm
echo ""
echo "To install:"
echo "  sudo dnf install ~/rpmbuild/RPMS/x86_64/ratpm-${VERSION}-${RELEASE}.*.rpm"
