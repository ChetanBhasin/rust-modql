// region:    --- OrderBy
#[derive(Debug)]
pub enum OrderBy {
	Asc(String),
	Desc(String),
}

impl core::fmt::Display for OrderBy {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
		match self {
			OrderBy::Asc(val) => {
				fmt.write_str(val)?;
				fmt.write_str(" ")?;
				fmt.write_str("ASC")?;
			}
			OrderBy::Desc(val) => {
				fmt.write_str(val)?;
				fmt.write_str(" ")?;
				fmt.write_str("DESC")?;
			}
		};

		Ok(())
	}
}

impl<T: AsRef<str>> From<T> for OrderBy {
	fn from(val: T) -> Self {
		let raw: &str = val.as_ref();

		if let Some(stripped) = raw.strip_prefix('!') {
			OrderBy::Desc(stripped.to_string())
		} else {
			OrderBy::Asc(raw.to_string())
		}
	}
}

// endregion: --- OrderBy

// region:    --- OrderBys
#[derive(Debug)]
pub struct OrderBys(Vec<OrderBy>);

impl OrderBys {
	pub fn new(v: Vec<OrderBy>) -> Self {
		OrderBys(v)
	}
	pub fn order_bys(self) -> Vec<OrderBy> {
		self.0
	}
}

// This will allow us to iterate over &OrderBys
impl<'a> IntoIterator for &'a OrderBys {
	type Item = &'a OrderBy;
	type IntoIter = std::slice::Iter<'a, OrderBy>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

// This will allow us to iterate over OrderBys directly (consuming it)
impl IntoIterator for OrderBys {
	type Item = OrderBy;
	type IntoIter = std::vec::IntoIter<OrderBy>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

// NOTE: If we want the Vec<T> and T, we have to make the individual from
//       specific to the type. Otherwise, conflict.

impl From<&str> for OrderBys {
	fn from(val: &str) -> Self {
		OrderBys(vec![val.into()])
	}
}
impl From<&String> for OrderBys {
	fn from(val: &String) -> Self {
		OrderBys(vec![val.into()])
	}
}
impl From<String> for OrderBys {
	fn from(val: String) -> Self {
		OrderBys(vec![val.into()])
	}
}

impl From<OrderBy> for OrderBys {
	fn from(val: OrderBy) -> Self {
		OrderBys(vec![val])
	}
}

impl<T: AsRef<str>> From<Vec<T>> for OrderBys {
	fn from(val: Vec<T>) -> Self {
		let d = val.into_iter().map(|o| OrderBy::from(o)).collect::<Vec<_>>();
		OrderBys(d)
	}
}

// endregion: --- OrderBys
