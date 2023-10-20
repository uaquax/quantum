use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref YES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![
        /* --- UA --- */
        "так".to_string(),
        "звісно".to_string(),
        "впевнено".to_string(),
        "безумовно".to_string(),
        "точно".to_string(),
        "вірно".to_string(),
        "відповідно".to_string(),
        "точно так".to_string(),
        "впевнено".to_string(),
        "звісно".to_string(),

        /* --- EN --- */
        "yes".to_string(),
        "yeah".to_string(),
        "yep".to_string(),
        "sure".to_string(),
        "absolutely".to_string(),
        "indeed".to_string(),
        "definitely".to_string(),
        "certainly".to_string(),
        "of course".to_string(),
        "for sure".to_string(),

        /* --- RU --- */
        "да".to_string(),
        "угу".to_string(),
        "конечно".to_string(),
        "ага".to_string(),
        "точно".to_string(),
        "верно".to_string(),
        "правда".to_string(),
        "безусловно".to_string(),
        "непременно".to_string(),
        "точно так".to_string(),
    ]));

    pub static ref NO: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![
        /* --- UA --- */
        "ні".to_string(),
        "не варто".to_string(),
        "зовсім ні".to_string(),
        "абсолютно ні".to_string(),
        "точно не".to_string(),
        "зовсім не".to_string(),
        "ні в якому разі".to_string(),
        "відмовляюсь".to_string(),
        "категорично ні".to_string(),
        "навряд чи".to_string(),

        /* --- EN --- */
        "no".to_string(),
        "nah".to_string(),
        "not really".to_string(),
        "negative".to_string(),
        "never".to_string(),
        "certainly not".to_string(),
        "definitely not".to_string(),
        "absolutely not".to_string(),
        "no way".to_string(),
        "of course not".to_string(),

        /* --- RU --- */
        "нет".to_string(),
        "неа".to_string(),
        "ни в коем случае".to_string(),
        "нисколько".to_string(),
        "вовсе нет".to_string(),
        "абсолютно нет".to_string(),
        "точно не".to_string(),
        "категорически нет".to_string(),
        "непременно нет".to_string(),
        "вряд ли".to_string(),
    ]));
}
