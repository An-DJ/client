/*
 *     Copyright 2023 The Dragonfly Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::Layer, prelude::*, EnvFilter, Registry};

pub fn init_logging(name: &str, dir: &str, level: Level) -> Vec<WorkerGuard> {
    let mut guards = vec![];
    LogTracer::init().expect("failed to init LogTracer");

    // Setup stdout layer.
    let (stdout_writer, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
    let stdout_logging_layer = Layer::new().with_writer(stdout_writer);
    guards.push(stdout_guard);

    // Setup file layer.
    let rolling_appender = RollingFileAppender::new(Rotation::DAILY, dir, name);
    let (rolling_writer, rolling_writer_guard) = tracing_appender::non_blocking(rolling_appender);
    let file_logging_layer = Layer::new().with_writer(rolling_writer);
    guards.push(rolling_writer_guard);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.to_string()));

    let subscriber = Registry::default()
        .with(env_filter)
        .with(stdout_logging_layer)
        .with(file_logging_layer);

    tracing::subscriber::set_global_default(subscriber).expect("failed to set global subscriber");

    guards
}
