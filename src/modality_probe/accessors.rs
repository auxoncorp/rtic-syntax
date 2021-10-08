use syn::Ident;

use crate::ast::{App, Local, LocalResource};
use crate::modality_probe::ast::ModalityProbeArgs;

impl App {
    /// Returns a vector of (task name, probe storage resource name, `Local` struct)
    pub fn modality_probe_storage_resources(&self) -> Vec<(&Ident, &Ident, &Local)> {
        std::iter::once(&self.init)
            .filter_map(move |task| {
                let task_name = &task.name;
                task.probe.as_ref().map(|pa| {
                    let (name, l) = pa.storage_declared_local_resource();
                    (task_name, name, l)
                })
            })
            .chain(self.idle.iter().filter_map(move |task| {
                let task_name = &task.name;
                task.probe.as_ref().map(|pa| {
                    let (name, l) = pa.storage_declared_local_resource();
                    (task_name, name, l)
                })
            }))
            .chain(
                self.hardware_tasks
                    .iter()
                    .filter_map(move |(task_name, task)| {
                        task.probe.as_ref().map(|pa| {
                            let (name, l) = pa.storage_declared_local_resource();
                            (task_name, name, l)
                        })
                    }),
            )
            .chain(
                self.software_tasks
                    .iter()
                    .filter_map(move |(task_name, task)| {
                        task.probe.as_ref().map(|pa| {
                            let (name, l) = pa.storage_declared_local_resource();
                            (task_name, name, l)
                        })
                    }),
            )
            .collect()
    }

    /// Returns a vector of (task name, probe resource name, `LocalResource` struct)
    pub fn modality_probe_local_resources(&self) -> Vec<(&Ident, &Ident, &LocalResource)> {
        std::iter::once(&self.init)
            .filter_map(move |task| {
                let task_name = &task.name;
                task.probe.as_ref().map(|pa| {
                    let (name, l) = pa.probe_local_resource();
                    (task_name, name, l)
                })
            })
            .chain(self.idle.iter().filter_map(move |task| {
                let task_name = &task.name;
                task.probe.as_ref().map(|pa| {
                    let (name, l) = pa.probe_local_resource();
                    (task_name, name, l)
                })
            }))
            .chain(
                self.hardware_tasks
                    .iter()
                    .filter_map(move |(task_name, task)| {
                        task.probe.as_ref().map(|pa| {
                            let (name, l) = pa.probe_local_resource();
                            (task_name, name, l)
                        })
                    }),
            )
            .chain(
                self.software_tasks
                    .iter()
                    .filter_map(move |(task_name, task)| {
                        task.probe.as_ref().map(|pa| {
                            let (name, l) = pa.probe_local_resource();
                            (task_name, name, l)
                        })
                    }),
            )
            .collect()
    }

    /// Returns a vector of (task name, `ModalityProbeArgs`)
    pub fn modality_probes(&self) -> Vec<(&Ident, &ModalityProbeArgs)> {
        std::iter::once(&self.init)
            .filter_map(move |task| task.probe.as_ref().map(|pa| (&task.name, pa)))
            .chain(
                self.idle
                    .iter()
                    .filter_map(move |task| task.probe.as_ref().map(|pa| (&task.name, pa))),
            )
            .chain(
                self.hardware_tasks
                    .iter()
                    .filter_map(move |(task_name, task)| {
                        task.probe.as_ref().map(|pa| (task_name, pa))
                    }),
            )
            .chain(
                self.software_tasks
                    .iter()
                    .filter_map(move |(task_name, task)| {
                        task.probe.as_ref().map(|pa| (task_name, pa))
                    }),
            )
            .collect()
    }
}
