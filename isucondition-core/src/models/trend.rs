use crate::models::isu::Isu;
use crate::models::isu_condition::IsuCondition;

pub type IsuWithCondition = (Isu, IsuCondition);

pub struct Trend {
    pub character: String,
    pub info: Vec<IsuWithCondition>,
    pub warning: Vec<IsuWithCondition>,
    pub critical: Vec<IsuWithCondition>,
}
