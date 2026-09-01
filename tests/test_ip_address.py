# stdlib
import os
import platform
from ipaddress import IPv4Address, IPv6Address

# 3rd party
import pytest

# this package
from local_ip_address import local_ip, local_ipv6


def test_ipv4():
	ipv4 = local_ip()
	print(ipv4)
	assert isinstance(ipv4, IPv4Address)
	assert str(ipv4) != "127.0.1.1"
	assert str(ipv4) != "127.0.0.1"


@pytest.mark.skipif(
		platform.system() == "Linux" and "GITHUB_ACTIONS" in os.environ,
		reason="No IPv6 support in Linux GitHub Actions",
		)
def test_ipv6():
	ipv6 = local_ipv6()
	print(ipv6)
	assert isinstance(ipv6, IPv6Address)
