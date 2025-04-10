mod mall;
use crate::mall::Mall;
use crate::mall::guard::Guard;
use crate::mall::floor::store::Store;
use crate::mall::floor::store::employee::Employee;


pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.clone())
        .max_by_key(|store| store.square_meters)
        .unwrap()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let all_employees: Vec<Employee> = mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .flat_map(|store| store.employees.clone())
        .collect();

    let max_salary = all_employees
        .iter()
        .map(|emp| emp.salary)
        .fold(0.0, f64::max);

    all_employees
        .into_iter()
        .filter(|emp| emp.salary == max_salary)
        .collect()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.employees.len())
        .sum::<usize>() + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<Guard>) {
    let total_square_meters: u64 = mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = (total_square_meters as f64 / 200.0).ceil() as usize;
    let current_guards = mall.guards.len();

    if required_guards > current_guards {
        let to_hire = required_guards - current_guards;
        for guard in guards.into_iter().take(to_hire) {
            mall.hire_guard(guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for emp in &mut store.employees {
                let hours = emp.working_hours.1 as i8 - emp.working_hours.0 as i8;
                if hours > 10 {
                    emp.raise(emp.salary * 0.10);
                } else {
                    emp.cut(emp.salary * 0.10);
                }
            }
        }
    }
}
