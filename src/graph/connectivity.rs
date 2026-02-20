use uuid::Uuid;

use super::{Connection, NodeId, PortName, Workflow};

impl Workflow {
    pub fn add_connection(
        &mut self,
        source: NodeId,
        target: NodeId,
        source_port: &PortName,
        target_port: &PortName,
    ) -> bool {
        if source == target {
            return false;
        }
        if self.path_exists(target, source) {
            return false;
        }
        if self.connections.iter().any(|c| {
            c.source == source
                && c.target == target
                && c.source_port == *source_port
                && c.target_port == *target_port
        }) {
            return false;
        }
        self.connections.push(Connection {
            id: Uuid::new_v4(),
            source,
            target,
            source_port: source_port.clone(),
            target_port: target_port.clone(),
        });
        true
    }

    fn path_exists(&self, from: NodeId, to: NodeId) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![from];

        while let Some(current) = stack.pop() {
            if current == to {
                return true;
            }

            if visited.insert(current) {
                self.connections
                    .iter()
                    .filter(|connection| connection.source == current)
                    .for_each(|connection| stack.push(connection.target));
            }
        }

        false
    }
}
