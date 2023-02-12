use self::{grade::Grade, attendance::Attendance};

mod grade;
mod attendance;

pub struct Class {
    grades: Vec<Grade>,
    attendance: Attendance,
}