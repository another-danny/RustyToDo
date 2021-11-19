pub struct ToDo {
	pub desc: String,
	pub status: Status	
}

pub enum Status {
	open,
	done
}