use actix::prelude::*;
use chrono::prelude::*;

#[derive(Message, Debug)]
#[rtype(Result = "()")]
struct SchedFailWarnMsg {
    name: String,
    description: Option<String>,
    receiver_dd_id: Option<String>,
    receiver_mail: String,
    error_time: DateTime<Utc>,
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

    let subscriber = MessageSubscriber::new().start();

    subscriber.do_send(msg)
}
