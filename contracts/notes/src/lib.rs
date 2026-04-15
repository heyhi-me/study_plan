#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct StudyPlan {
    pub id: u64,
    pub subject: String,
    pub deadline: String,
    pub total_sessions: u32,
    pub done_sessions: u32,
    pub duration_mins: u32,
    pub priority: u32, // 1=rendah, 2=sedang, 3=tinggi
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct StudyStats {
    pub total_plans: u32,
    pub total_sessions_done: u32,
    pub total_sessions_all: u32,
}

const PLAN_DATA: Symbol = symbol_short!("PLAN_DATA");

#[contract]
pub struct StudyPlanContract;

#[contractimpl]
impl StudyPlanContract {
    // Ambil semua study plan
    pub fn get_plans(env: Env) -> Vec<StudyPlan> {
        env.storage()
            .instance()
            .get(&PLAN_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Buat study plan baru
    pub fn create_plan(
        env: Env,
        subject: String,
        deadline: String,
        total_sessions: u32,
        duration_mins: u32,
        priority: u32,
    ) -> String {
        let mut plans: Vec<StudyPlan> = env
            .storage()
            .instance()
            .get(&PLAN_DATA)
            .unwrap_or(Vec::new(&env));

        let plan = StudyPlan {
            id: env.prng().gen::<u64>(),
            subject,
            deadline,
            total_sessions,
            done_sessions: 0,
            duration_mins,
            priority: priority.clamp(1, 3),
        };

        plans.push_back(plan);
        env.storage().instance().set(&PLAN_DATA, &plans);

        String::from_str(&env, "Study plan berhasil dibuat")
    }

    // Tandai satu sesi sebagai selesai
    pub fn mark_session_done(env: Env, id: u64) -> String {
        let mut plans: Vec<StudyPlan> = env
            .storage()
            .instance()
            .get(&PLAN_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..plans.len() {
            let mut plan = plans.get(i).unwrap();
            if plan.id == id {
                if plan.done_sessions >= plan.total_sessions {
                    return String::from_str(&env, "Semua sesi sudah selesai");
                }
                plan.done_sessions += 1;
                plans.set(i, plan);
                env.storage().instance().set(&PLAN_DATA, &plans);
                return String::from_str(&env, "Sesi berhasil ditandai selesai");
            }
        }

        String::from_str(&env, "Study plan tidak ditemukan")
    }

    // Ambil progress (0-100) untuk plan tertentu
    pub fn get_progress(env: Env, id: u64) -> u32 {
        let plans: Vec<StudyPlan> = env
            .storage()
            .instance()
            .get(&PLAN_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..plans.len() {
            let plan = plans.get(i).unwrap();
            if plan.id == id {
                if plan.total_sessions == 0 {
                    return 0;
                }
                return (plan.done_sessions * 100) / plan.total_sessions;
            }
        }

        0
    }

    // Filter plans berdasarkan priority (1/2/3)
    pub fn get_plans_by_priority(env: Env, priority: u32) -> Vec<StudyPlan> {
        let plans: Vec<StudyPlan> = env
            .storage()
            .instance()
            .get(&PLAN_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);
        for i in 0..plans.len() {
            let plan = plans.get(i).unwrap();
            if plan.priority == priority {
                result.push_back(plan);
            }
        }

        result
    }

    // Ambil statistik keseluruhan
    pub fn get_stats(env: Env) -> StudyStats {
        let plans: Vec<StudyPlan> = env
            .storage()
            .instance()
            .get(&PLAN_DATA)
            .unwrap_or(Vec::new(&env));

        let mut total_done = 0u32;
        let mut total_all = 0u32;

        for i in 0..plans.len() {
            let plan = plans.get(i).unwrap();
            total_done += plan.done_sessions;
            total_all += plan.total_sessions;
        }

        StudyStats {
            total_plans: plans.len(),
            total_sessions_done: total_done,
            total_sessions_all: total_all,
        }
    }

    // Hapus study plan berdasarkan id
    pub fn delete_plan(env: Env, id: u64) -> String {
        let mut plans: Vec<StudyPlan> = env
            .storage()
            .instance()
            .get(&PLAN_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..plans.len() {
            if plans.get(i).unwrap().id == id {
                plans.remove(i);
                env.storage().instance().set(&PLAN_DATA, &plans);
                return String::from_str(&env, "Study plan berhasil dihapus");
            }
        }

        String::from_str(&env, "Study plan tidak ditemukan")
    }
}

mod test;