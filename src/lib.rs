use actix_web::Result;
use std::{ffi::OsString, time::Duration};
use windows_service::{define_windows_service, service_dispatcher};
use windows_service::{
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
};

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

mod app;

const SERVICE_NAME: &str = "DropIt";

pub fn run() -> Result<(), windows_service::Error> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
        .build("/Users/juanl/dev/drop-it/log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        log::info!("Service error: {}", e);

        std::process::exit(1);
    }
}

fn run_service(_arguments: Vec<OsString>) -> Result<(), windows_service::Error> {
    let sys = actix_web::rt::System::new(SERVICE_NAME);

    if let Err(e) = app::run_server() {
        return Err(windows_service::Error::Winapi(e));
    };

    let (mut send_stop, recv_stop) = {
        let (p, c) = futures::channel::oneshot::channel::<()>();
        (Some(p), c)
    };

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            ServiceControl::Stop => {
                send_stop.take().unwrap().send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(service_status_running())?;

    actix_web::rt::spawn(async move {
        recv_stop.await.unwrap();
        status_handle
            .set_service_status(service_status_stopped())
            .unwrap();

        actix_web::rt::System::current().stop()
    });

    sys.run().unwrap();

    Ok(())
}

fn service_status_running() -> ServiceStatus {
    ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }
}

fn service_status_stopped() -> ServiceStatus {
    ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    }
}
