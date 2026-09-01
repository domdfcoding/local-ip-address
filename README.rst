=================
local-ip-address
=================

.. start short_desc

**Wrapper around the local-ip-address Rust crate to retrieve local IP addresses.**

.. end short_desc


.. list-table::
	:stub-columns: 1
	:widths: 10 90

	* - Tests
	  - |actions_linux| |actions_windows| |actions_macos|
	* - PyPI
	  - |pypi-version| |supported-versions| |supported-implementations| |wheel|
	* - Activity
	  - |commits-latest| |commits-since| |maintained| |pypi-downloads|
	* - QA
	  - |codefactor| |actions_flake8| |actions_mypy|
	* - Other
	  - |license| |language| |requires|

.. |actions_linux| image:: https://github.com/domdfcoding/local-ip-address/workflows/Linux/badge.svg
	:target: https://github.com/domdfcoding/local-ip-address/actions?query=workflow%3A%22Linux%22
	:alt: Linux Test Status

.. |actions_windows| image:: https://github.com/domdfcoding/local-ip-address/workflows/Windows/badge.svg
	:target: https://github.com/domdfcoding/local-ip-address/actions?query=workflow%3A%22Windows%22
	:alt: Windows Test Status

.. |actions_macos| image:: https://github.com/domdfcoding/local-ip-address/workflows/macOS/badge.svg
	:target: https://github.com/domdfcoding/local-ip-address/actions?query=workflow%3A%22macOS%22
	:alt: macOS Test Status

.. |actions_flake8| image:: https://github.com/domdfcoding/local-ip-address/workflows/Flake8/badge.svg
	:target: https://github.com/domdfcoding/local-ip-address/actions?query=workflow%3A%22Flake8%22
	:alt: Flake8 Status

.. |actions_mypy| image:: https://github.com/domdfcoding/local-ip-address/workflows/mypy/badge.svg
	:target: https://github.com/domdfcoding/local-ip-address/actions?query=workflow%3A%22mypy%22
	:alt: mypy status

.. |requires| image:: https://dependency-dash.repo-helper.uk/github/domdfcoding/local-ip-address/badge.svg
	:target: https://dependency-dash.repo-helper.uk/github/domdfcoding/local-ip-address/
	:alt: Requirements Status

.. |coveralls| image:: https://img.shields.io/coveralls/github/domdfcoding/local-ip-address/master?logo=coveralls
	:target: https://coveralls.io/github/domdfcoding/local-ip-address?branch=master
	:alt: Coverage

.. |codefactor| image:: https://img.shields.io/codefactor/grade/github/domdfcoding/local-ip-address?logo=codefactor
	:target: https://www.codefactor.io/repository/github/domdfcoding/local-ip-address
	:alt: CodeFactor Grade

.. |pypi-version| image:: https://img.shields.io/pypi/v/local-ip-address
	:target: https://pypi.org/project/local-ip-address/
	:alt: PyPI - Package Version

.. |supported-versions| image:: https://img.shields.io/pypi/pyversions/local-ip-address?logo=python&logoColor=white
	:target: https://pypi.org/project/local-ip-address/
	:alt: PyPI - Supported Python Versions

.. |supported-implementations| image:: https://img.shields.io/pypi/implementation/local-ip-address
	:target: https://pypi.org/project/local-ip-address/
	:alt: PyPI - Supported Implementations

.. |wheel| image:: https://img.shields.io/pypi/wheel/local-ip-address
	:target: https://pypi.org/project/local-ip-address/
	:alt: PyPI - Wheel

.. |license| image:: https://img.shields.io/github/license/domdfcoding/local-ip-address
	:target: https://github.com/domdfcoding/local-ip-address/blob/master/LICENSE
	:alt: License

.. |language| image:: https://img.shields.io/github/languages/top/domdfcoding/local-ip-address
	:alt: GitHub top language

.. |commits-since| image:: https://img.shields.io/github/commits-since/domdfcoding/local-ip-address/v0.1.0.post1
	:target: https://github.com/domdfcoding/local-ip-address/pulse
	:alt: GitHub commits since tagged version

.. |commits-latest| image:: https://img.shields.io/github/last-commit/domdfcoding/local-ip-address
	:target: https://github.com/domdfcoding/local-ip-address/commit/master
	:alt: GitHub last commit

.. |maintained| image:: https://img.shields.io/maintenance/yes/2026
	:alt: Maintenance

.. |pypi-downloads| image:: https://img.shields.io/pypi/dm/local-ip-address
	:target: https://pypistats.org/packages/local-ip-address
	:alt: PyPI - Downloads


Installation
--------------

.. start installation

``local-ip-address`` can be installed from PyPI.

To install with ``pip``:

.. code-block:: bash

	$ python -m pip install local-ip-address

.. end installation


Usage
--------------

``local_ip_address.local_ip() -> ipaddress.IPv4Address``
===========================================================

Retrieves the local IPv4 address of the machine in the local network.


``local_ip_address.local_ipv6() -> ipaddress.IPv6Address``
===========================================================

Retrieves the local IPv6 address of the machine in the local network.
