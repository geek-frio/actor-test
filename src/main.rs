use actix::prelude::*;
use chrono::prelude::*;

#[derive(Message, Debug)]
#[rtype(Result = "()")]
struct SchedFailWarnMsg {
    name: String,
    description: Option<String>,
    receiver_dd_id: Option<String>,
    receiver_mail: String,
    error_time: DateTime<Local>,
}

#[derive(Message, Debug)]
#[rtype(Result = "()")]
struct SubcribeMsg(Recipient<SchedFailWarnMsg>);

struct MessageSubscriber {
    recvs: Vec<Recipient<SchedFailWarnMsg>>,
}

impl MessageSubscriber {
    fn new() -> MessageSubscriber {
        MessageSubscriber { recvs: vec![] }
    }
}

impl Actor for MessageSubscriber {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("MessageSubscriber is started!");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("MessageSubscriber is stopped!");
    }
}

impl Handler<SubcribeMsg> for MessageSubscriber {
    type Result = ();

    fn handle(&mut self, msg: SubcribeMsg, ctx: &mut Self::Context) -> Self::Result {
        self.recvs.push(msg.0);
    }
}

impl Handler<SchedFailWarnMsg> for MessageSubscriber {
    type Result = ();

    fn handle(&mut self, msg: SchedFailWarnMsg, ctx: &mut Self::Context) -> Self::Result {}
}

impl Actor for SmsReceiver {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("SmsReceiver is started!");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("SmsReceiver is stopped!");
    }
}

impl Handler<SchedFailWarnMsg> for SmsReceiver {
    type Result = ();
    fn handle(&mut self, msg: SchedFailWarnMsg, ctx: &mut Self::Context) -> Self::Result {}
}
struct SmsReceiver;

impl Actor for DingDingReceiver {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {}

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

impl Handler<SchedFailWarnMsg> for DingDingReceiver {
    type Result = ();

    fn handle(&mut self, msg: SchedFailWarnMsg, ctx: &mut Self::Context) -> Self::Result {}
}
struct DingDingReceiver;

fn main() {
    let system = System::new("events");
    let sms_rec = SmsReceiver {}.start().recipient();
    let dd_rec = DingDingReceiver {}.start().recipient();

    let sm_rec = SubcribeMsg(sms_rec);
    let dd_red = SubcribeMsg(dd_rec);

    let subscriber = MessageSubscriber::new().start();
    subscriber.do_send(sm_rec);
    subscriber.do_send(dd_red);

    let sch_msg = SchedFailWarnMsg {
        name: "test".to_string(),
        description: Some("description".to_string()),
        receiver_dd_id: Some("receiver_dd_id".to_string()),
        receiver_mail: "receiver mail".to_string(),
        error_time: Local::now(),
    };
    subscriber.do_send(sch_msg);
}
