// Import the required Substrate smart contract libraries
use ink_lang::contract;

// Define the contract and its methods
#[contract]
pub mod parent_teacher_conference {
    use ink_prelude::vec::Vec;
    
    #[ink(storage)]
    pub struct ParentTeacherConference {
        // Define the state variables here
        appointments: Vec<Appointment>,
    }
    
    impl ParentTeacherConference {
        // Define the contract's methods here
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                appointments: Vec::new(),
            }
        }
        
        #[ink(message)]
        pub fn schedule_appointment(&mut self, parent: String, teacher: String, date: String) {
            // Define the logic for scheduling appointments here
            let new_appointment = Appointment {
                parent: parent,
                teacher: teacher,
                date: date,
            };
            self.appointments.push(new_appointment);
        }
    }
    
    struct Appointment {
        parent: String,
        teacher: String,
        date: String,
    }
}


// contract LessonPlan {
//     lesson_plans: Vec<String>,
//     owner: address,
// }

impl LessonPlan {
    pub fn new(owner: address) -> LessonPlan {
        LessonPlan {
            lesson_plans: Vec::new(),
            owner: owner,
        }
    }

    pub fn add_lesson_plan(&mut self, lesson_plan: String) {
        self.lesson_plans.push(lesson_plan);
    }

    pub fn get_lesson_plans(&self) -> Vec<String> {
        self.lesson_plans.clone()
    }
}