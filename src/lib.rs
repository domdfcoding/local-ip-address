use local_ip_address::{local_ip, local_ipv6};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

/// Wrapper around the local-ip-address Rust crate to retrieve local IP addresses.

#[pyfunction(name = "local_ip")]
/// Retrieves the local IPv4 address of the machine in the local network.
pub fn local_ip_py() -> PyResult<String> {
	let capabilities = local_ip();
	match capabilities {
		Ok(ip) => Ok(ip.to_string()),
		Err(e) => Err(PyRuntimeError::new_err(e.to_string())), // TODO: better error
	}
}

#[pyfunction(name = "local_ipv6")]
/// Retrieves the local IPv6 address of the machine in the local network.
pub fn local_ipv6_py() -> PyResult<String> {
	let capabilities = local_ipv6();
	match capabilities {
		Ok(ip) => Ok(ip.to_string()),
		Err(e) => Err(PyRuntimeError::new_err(e.to_string())), // TODO: better error
	}
}

// fn ip_to_str(ip: IpAddr) -> String {
// 	match ip {
// 		IpAddr::V4(ipv4) => ipv4.to_string(),
// 		IpAddr::V6(ipv6) => ipv6.to_string(),
// 	}
// }

#[pymodule]
fn _local_ip_address(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
	let local_ip = wrap_pyfunction!(local_ip_py, m)?;
	local_ip.setattr("__module__", "local_ip_address")?;
	m.add_function(local_ip).unwrap();

	let local_ipv6 = wrap_pyfunction!(local_ipv6_py, m)?;
	local_ipv6.setattr("__module__", "local_ip_address")?;
	m.add_function(local_ipv6).unwrap();

	Ok(())
}
