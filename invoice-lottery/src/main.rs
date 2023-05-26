use std::collections::HashSet;
use std::collections::HashMap;
use rand::Rng;
extern crate lottery_chk;
use lottery_chk::lottery::checker;
use lottery_chk::lottery::drawing;
use lottery_chk::lottery::envreader;
use std::time::Instant;
use std::thread;
use std::sync::{ Arc, Barrier };
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver };
const MAX_NUMBER: u32 = 99999999;
const INT_NUM: u32 = 93410000;
const USER_NUM: u32 = 100;
const TICKETS_NUM: u32 = 100;
const NTHREADS: u32 = 2;

fn main() {
    println!("Hello, 統一發票模擬器");
    let now = Instant::now();
    let mut children = vec![];
    let mut ids = Vec::with_capacity(10);
    let thread_nun = envreader::got_env_number("THREAD_NUM".to_string(),1, 64, NTHREADS, 0) as usize;
    let barrier = Arc::new(Barrier::new(thread_nun));
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    for i in 0..thread_nun {
        let c = barrier.clone();
        let thread_tx = tx.clone();
        // Spin up another thread
        children.push(
            thread::spawn(move || {
                checking_invoce(thread_tx, c, i);
            })
        );
    }
    for _ in 0..thread_nun {
        ids.push(rx.recv());
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    let elapsed = now.elapsed();
    println!("Time spent : {:.1?}", elapsed);
    let ids = ids.into_iter().flatten().collect::<Vec<String>>();
    for inx in 0..thread_nun {
        println!("\n======== 測試第 {} 次結果如下 ========\n{}", inx + 1, ids[inx]);
    }
}

fn checking_invoce(tx: Sender<String>, waiting: Arc<Barrier>, th_id: usize) {
    //為了讓輸出乾淨，只有第一個執行緒會印出錯誤訊息 th_id=0 ，其他執行緒只要讀數值即可
    let mut int_num = envreader::got_env_number("INT_NUMBER".to_string(), 1, MAX_NUMBER, INT_NUM, th_id);
    let user_num  = envreader::got_env_number("USER_NUMBER".to_string(),1 ,1000000, USER_NUM, th_id);
    let ticket_num = envreader::got_env_number("TICKETS_NUM".to_string(),1 ,1000000, TICKETS_NUM, th_id);
    let mut count: u32 = 1;
    let mut lucky_inv_map = HashMap::new();
    let mut luckyone = HashSet::new();
    let mut one_prize: u32;
    let mut one_wins_ticket: u32;
    let mut max_prize: u32 = 0;
    let mut all_prize: u32 = 0;
    println!(
        "執行緒#{} : 由{} 開始對發票，共{}人，每人{}張，開始 ",
        th_id,
        int_num,
        user_num,
        ticket_num
    );
    for i in 1..user_num + 1 {
        //println!("user:{:03} checking..",i);
        one_prize = 0;
        one_wins_ticket = 0;
        for _j in 1..ticket_num + 1 {
            for winning in drawing::Drawing2305::drawingresult().iter() {
                let prize = checker::check_winning(&winning, int_num + count);
                if prize > 0 {
                    let show_prize = || {
                        println!(
                            "Lucky user,{:03}, thread#{}, Voice num, {:08}, win, {}",
                            i,
                            th_id,
                            int_num + count,
                            prize
                        )
                    };
                    show_prize();
                    luckyone.insert(i);
                    one_prize = one_prize + prize;
                    let vcount = lucky_inv_map.entry(prize).or_insert(0);
                    *vcount += 1;
                    one_wins_ticket += 1;
                    if max_prize < prize {
                        max_prize = prize;
                    }
                    break;
                }
            }
            count = count + rand::thread_rng().gen_range(2..9999);
            if int_num + count > MAX_NUMBER {
                int_num = 1;
            }
        }
        all_prize = all_prize + one_prize;
        if one_wins_ticket > 1 {
            println!("* Very lucky user{:03} thread#{} wins, {}",i,th_id, one_prize);
        }
    }
    waiting.wait();
    thread::sleep(Duration::from_millis(100 * (th_id as u64)));
    let mut report = format!("本次模擬 {} 人, 每人對獎{}張\n", user_num, ticket_num);
    let mut zero_tickets = ticket_num * user_num;
    let mut keys: Vec<&u32> = lucky_inv_map.keys().collect();
    keys.sort();
    //for (key, value) in lucky_inv_map.into_iter() {
    for key in keys {
        report = format!("{}發票獎金{} : 有{}張\n", report, key, lucky_inv_map[key]);
        zero_tickets = zero_tickets - lucky_inv_map[key];
    }
    report = format!("{}本次有{}中獎人 \n", report, luckyone.len());
    report = format!("{}總獎金{}元\n", report, all_prize);
    report = format!("{}最大獎{}元\n", report, max_prize);
    report = format!("{}沒中獎發票{}張\n", report, zero_tickets);
    tx.send(report).unwrap();
    println!("##執行緒 {} 完畢##", th_id);
}