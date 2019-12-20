use chrono;
use chrono::prelude::*;
use itertools::Itertools;
use serde::Deserialize;
use std::cmp::Ord;

//#[derive(Clone, Deserialize, Debug)]
//pub struct Mirrors {
    //cutoff: Option<u64>,
    //last_check: Option<DateTime<Utc>>,
    //num_checks: Option<u64>,
    //check_frequency: Option<u64>,
    //pub urls: Vec<Mirror>,
    //version: u64,
//}

//impl Mirrors {
    //pub fn get(&self) -> impl Iterator<Item = &Mirror> {
        //self.urls
            //.iter()
            //.filter(|&s| s.protocol == Protocol::Https)
            //.sorted_by(|a, b| {
                //a.score
                    //.partial_cmp(&b.score)
                    //.unwrap_or(std::cmp::Ordering::Equal)
            //})
            //.sorted_by(|a, b| {
                //b.completion_pct
                    //.partial_cmp(&a.completion_pct)
                    //.unwrap_or(std::cmp::Ordering::Equal)
            //})
            //.sorted_by(|a, b| Ord::cmp(&b.last_sync, &a.last_sync))
    //}

    //pub async fn rate(&self) {
        //let mut tasks: Vec<JoinHandle<(String, f64)>> = Vec::new();
        //for mirror in self.get().take(20) {
            //let url = mirror.url.clone();
            //let db_url = mirror.get_coredb_url();
            //tasks.push(task::spawn(async move {
                //let now = Instant::now();
                //let mut res = surf::get(db_url).await.unwrap();
                //let n_bytes = res.body_bytes().await.unwrap();
                //let n_bytes = 0.0f64;
                //let elapsed = now.elapsed();
                //(url, n_bytes as f64 / elapsed.as_secs_f64())
            //}));
        //}

        //for task in tasks {
            //let (url, rate) = task.await;
            //println!("Rate: {:6.2} {}", rate / 1000.0, url);
        //}
    //}
//}
