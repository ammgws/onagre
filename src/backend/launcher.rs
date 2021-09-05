use iced::futures;
use iced::futures::channel::mpsc;
use iced_native::futures::stream::BoxStream;
use iced_native::futures::StreamExt;
use iced_native::Subscription;
use std::hash::Hash;
use pop_launcher::{json_input_stream, Response, Request};
use iced::futures::channel::mpsc::{channel};
use iced::futures::channel::mpsc::Sender;
use async_process::{ChildStdin, Command, ChildStdout, ChildStderr};
use std::process::Stdio;
use iced::futures::{SinkExt, AsyncWriteExt};
use iced::futures::io::BufReader;
use crate::backend::launcher::futures::join;
use iced::futures::AsyncBufReadExt;
use iced::futures::stream;
use crate::backend::PopRequest;

// Whenever a message is red from pop-laucnher stdout, send it to the subscription receiver
async fn handle_stdout(stdout: ChildStdout, mut sender: Sender<Response>) {
    debug!("Handling stdout");

    let mut stream = json_input_stream::<_, Response>(stdout);

    while let Some(response) = stream.next().await {
        debug!("Got response from pop {:?}", response);
        sender.send(response.unwrap()).await.unwrap();
    }
}

// Whenever a message is red from pop-laucnher stderr, print it to onagre stderr
async fn handle_stderr(stderr: ChildStderr) {
    debug!("Handling stderr");

    let mut lines = BufReader::new(stderr).lines();

    while let Some(line) = lines.next().await {
        error!("{}", line.unwrap());
    }
}

// Listen for incoming `pop_launcher::Request` from the receiver and forward them to
// pop launcher stdin
async fn handle_stdin(mut stdin: ChildStdin, mut request_rx: mpsc::Receiver<String>) {
    while let Some(request) = request_rx.next().await {
        let request = Request::Search(request);
        let request = serde_json::to_string(&request).unwrap();
        let request = format!("{}\n", request);
        stdin.write(request.as_bytes()).await.unwrap();
        debug!("wrote request {:?} to pop-launcher stdin", request);
        stdin.flush().await.unwrap();
    }
}

pub struct PopLauncherSubscription {
    id: String,
}

#[derive(Debug, Clone)]
pub enum PopMessage {
    Ready(Sender<PopResponse>),
    Message(String),
}

impl PopLauncherSubscription {
    pub fn subscription() -> Subscription<PopMessage> {
        iced::Subscription::from_recipe(PopLauncherSubscription {
            id: "pop-sub".to_string(),
        })
    }
}

impl<H, I> iced_native::subscription::Recipe<H, I> for PopLauncherSubscription
    where
        H: std::hash::Hasher,
{
    type Output = PopMessage;

    fn hash(&self, state: &mut H) {
        std::any::TypeId::of::<Self>().hash(state);
        self.id.hash(state)
    }

    fn stream(self: Box<Self>, _: BoxStream<I>) -> BoxStream<Self::Output> {
        debug!("Starting `pop-launcher` subscription");
        let mut child = Command::new("pop-launcher")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let (response_tx, response_rx) = channel(32);
        let (request_tx, request_rx) = channel(32);

        let stdout = child.stdout.unwrap();
        let stderr = child.stderr.unwrap();
        let stdin = child.stdin.unwrap();


        let stdout_handle = handle_stdout(stdout, response_tx);
        let stderr_handle = handle_stderr(stderr);
        let stdin_handle = handle_stdin(stdin, request_rx);

        async_std::task::spawn(async {
            join!(stdout_handle, stderr_handle, stdin_handle);
        });


        let pop_response_rx = response_rx.map(|response| match response {
            Response::Close => "Close".to_string(),
            Response::Context { .. } => "Context".to_string(),
            Response::DesktopEntry { .. } => "DesktopEntry".to_string(),
            Response::Update(entries) => entries.iter().map(|e| e.name.clone()).collect::<Vec<String>>().join("\n"),
            Response::Fill(_) => "Fill".to_string(),
        })
            .map(|r| PopMessage::Message(r));

        Box::pin( stream::iter(vec![PopMessage::Ready(request_tx)])
            .chain(pop_response_rx))
    }
}


