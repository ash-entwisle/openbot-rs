use serde::{Serialize, Deserialize};
use std::fs::read_to_string;
use std::env;
use std::sync::{Mutex, Arc, RwLock};
use toml::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub name: Option<String>,
    pub prefix: Option<String>,
    pub description: Option<String>,
    pub invite: Option<String>,
    pub server: Option<String>,
    pub website: Option<String>,
    pub docs: Option<String>,
    pub github: Option<String>,
    pub pfp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embed {
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Admin {
    pub users: Option<Vec<String>>,
    pub guilds: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub info: Info,
    pub embed: Embed,
    pub admin: Admin,  
}

impl Config {
    pub fn get() -> Config {
        let lock: Arc<Mutex<Config>> = CONFIG.with(|c: &RwLock<Arc<Mutex<Config>>>| c.read().unwrap().clone());
        let config: Config = lock.lock().unwrap().clone();
        drop(lock);
        config
    }

    pub fn set(path: String) -> Config {
        let path: String = env::current_dir()
            .expect("Failed to get current directory")
            .display()
            .to_string() 
            +"/" 
            +path.as_str();
        
        let file: String = read_to_string(&path)
            .expect(format!("Failed to read file: {}", path).as_str());

        let config: Config = from_str(&file)
            .expect(format!("Failed to parse file: {}", path).as_str());

        CONFIG.with(|c: &RwLock<Arc<Mutex<Config>>>| *c.write().unwrap() = Arc::new(Mutex::new(config)));

        Config::get()
    }
}

thread_local! {
    static CONFIG: RwLock<Arc<Mutex<Config>>> = RwLock::new(Arc::new(Mutex::new(Config {
        info: Info {
            name: None,
            prefix: None,
            description: None,
            invite: None,
            server: None,
            website: None,
            docs: None,
            github: None,
            pfp: None,
        },
        embed: Embed {
            color: None,
        },
        admin: Admin {
            users: None,
            guilds: None,
        },
    })));
}