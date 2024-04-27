use Max_priority_queue::{SmartPriorityQueue, Element, PriorityUsize};
use rand;
#[derive(Debug)]
struct JobExecution {
    id: usize,
    job_name: &'static str
}

#[derive(Debug, Copy, Clone)]
enum Urgency {
    Mininum,
    Medium,
    Maximum
}
impl PriorityUsize for Urgency {
    fn to_usize(self: &Self) -> usize {
        match self {
            Self::Mininum => 1,
            Self::Medium => 2,
            Self::Maximum => 3,
        }
    }
}

fn main() {

    let mut last_id_execution = 1000;

    // Jobs:

    let jobs = ["job1", "job2", "job3", "job4", "job5", "job6", "job7", "job8", "job9", "job10"];

    let mut elements = vec![];

    let mut priority;

    for _i in 1..=100 {
        priority = match rand::random::<u8>() % 3 {
            0 => Urgency::Mininum,
            1 => Urgency::Medium,
            2 => Urgency::Maximum,
            _ => panic!("Impossível!!!")
        };
        elements.push(Element::new(priority, Box::new(JobExecution{id: last_id_execution + 1, job_name: jobs[rand::random::<usize>() % 10]})));
        last_id_execution += 1;
    }

    println!("Os 10 primeiros elementos a entrarem na fila são:");
    for i in 0..10 {
        println!("{:?}", elements.get(i).unwrap());
    }

    let mut priority_queue = SmartPriorityQueue::new(elements);

    println!("Os três primeiros jobs a serem executados são:");
    for i in 1..=3 {
        println!("{:?}", priority_queue.extract_max_priority().unwrap());
    }

    // Vamos deletar alguns jobs. Se o elemento estiver na fila, o elemento é deletado da fila e é printada uma mensagem de sucesso.
    // Caso conrário é printada uma mensagem que o elemento não consta na fila.
    // Isso quer dizer que ele nunca foi inserido ou já foi deletado ou executado.

    priority_queue.delete(14);

    priority_queue.delete(97);

    priority_queue.delete(51);

    priority_queue.delete(200);  // nunca foi inserido


    // Vamos inserir uns elementos novos

    priority_queue.insert(Element::new(Urgency::Maximum,
                                       Box::new(JobExecution {id: last_id_execution + 1, job_name: "job4"})));
    last_id_execution += 1;

    priority_queue.insert(Element::new(Urgency::Maximum,
                                       Box::new(JobExecution {id: last_id_execution + 1, job_name: "job4"})));
    last_id_execution += 1;

    priority_queue.insert(Element::new(Urgency::Maximum,
                                       Box::new(JobExecution {id: last_id_execution + 1, job_name: "job4"})));
    last_id_execution += 1;


    // Vamos mudar a prioridade de um job:

    println!("Vamos alterar algumas prioridades:");

    priority_queue.change_priority(15, Urgency::Maximum);

    priority_queue.change_priority(97, Urgency::Medium);

    priority_queue.change_priority(62, Urgency::Mininum);

    priority_queue.change_priority(2, Urgency::Mininum);


}
