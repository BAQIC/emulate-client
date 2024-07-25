# emulate-client
This is a client for the Quantum Emulator Server. It allows you to add, get, update, and remove agents, submit tasks, and get the results of tasks.

## Usage
### Get help
```bash
cargo run -- -h
```

### Add an agent
The agent include the following fields:
- `ip`: The IP address of the agent.
- `hostname`: The hostname of the agent. If you don't know the ip address, you can use the hostname.
- `port`: The port of the agent.
- `qubit_count`: The number of qubits the agent has.
- `circuit_depth`: The maximum depth of the circuit the agent can handle.

An example of the return value of the agent:
```bash
{
  "agent": {
    "circuit_depth": 10,
    "id": "6a2b6059-fcd0-44a1-8bd8-2934ffa0ffa2",
    "ip": "::1",
    "port": 8080,
    "qubit_count": 10,
    "qubit_idle": 10,
    "status": "Running"
  }
}
```

You can use the following command to add an agent:
```bash
# with hostname
cargo run -- -m add-agent --agent-hostname localhost --agent-port 8080 --agent-qubit-count 10 --agent-circuit-depth 10
# with ip
cargo run -- -m add-agent --agent-ip 127.0.0.1 --agent-port 8080 --agent-qubit-count 10 --agent-circuit-depth 10
```

### Get agents
You can use `ip` or `hostname` to get the agents. And you can use `port` to filter the agents. It will return all information of relative agents. The return value:
```bash
{
  "agents": [
    {
      "circuit_depth": 20,
      "id": "964394dd-a70b-4375-90eb-5dc5f45b2081",
      "ip": "172.18.0.4",
      "port": 3003,
      "qubit_count": 20,
      "qubit_idle": 20,
      "status": "Running"
    }
  ]
}
```

```bash
# get all agents with same ip address
cargo run -- -m get-agents --agent-ip 172.18.0.4
# get all agents with same hostname
cargo run -- -m get-agents --agent-hostname localhost
# get agent with specific ip and port
cargo run -- -m get-agents --agent-ip 172.18.0.4 --agent-port 8080
# get agent with specific hostname and port
cargo run -- -m get-agents --agent-hostname localhost --agent-port 8080
```

### Update an agent
You can use `id` to upadte the agent. And you can update the `ip`, `port`, `qubit_count`, `circuit_depth` and `status`. You can get the `id` from the `get-agents` command. The return value:
```bash
{
  "agent": {
    "circuit_depth": 20,
    "id": "964394dd-a70b-4375-90eb-5dc5f45b2081",
    "ip": "172.18.0.4",
    "port": 3003,
    "qubit_count": 20,
    "qubit_idle": 20,
    "status": "Down"
  }
}
```

You can use the following command to update the agent:
```bash
# update the status of the agent
cargo run -- -m update-agent --agent-id 964394dd-a70b-4375-90eb-5dc5f45b2081 --agent-status down

# update the ip of the agent
cargo run -- -m update-agent --agent-id 964394dd-a70b-4375-90eb-5dc5f45b2081 --agent-ip 127.0.0.1

# update the port of the agent
cargo run -- -m update-agent --agent-id 964394dd-a70b-4375-90eb-5dc5f45b2081 --agent-port 8080

# update the qubit count of the agent
cargo run -- -m update-agent --agent-id 964394dd-a70b-4375-90eb-5dc5f45b2081 --agent-qubit-count 10

# update the circuit depth of the agent
cargo run -- -m update-agent --agent-id 964394dd-a70b-4375-90eb-5dc5f45b2081 --agent-circuit-depth 10
```

### Remove an agent
You can use `id` to remove the agent. You can get the `id` from the `get-agents` command. The return value:
```bash
{
  "agent": {
    "circuit_depth": 20,
    "id": "964394dd-a70b-4375-90eb-5dc5f45b2081",
    "ip": "127.0.0.1",
    "port": 8080,
    "qubit_count": 10,
    "qubit_idle": 20,
    "status": "Down"
  }
}
```

You can use the following command to remove an agent:
```bash
cargo run -- -m remove-agent --agent-id 964394dd-a70b-4375-90eb-5dc5f45b2081
```

### Submit a task
The task include the following fields:
- `code`: The code of the task.
- `shots`: The number of shots.
- `depth`: The depth of the circuit.
- `qubits`: The number of qubits.

An example of the return value of the submission:
```bash
{
  "task": {
    "created_time": "2024-07-25T14:43:48.041348",
    "depth": 4,
    "exec_shots": 0,
    "id": "2de9ab58-7a4a-44de-837d-280d1d50dd89",
    "qubits": 2,
    "result": null,
    "shots": 3000,
    "source": "OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[2];\ncreg c[2];\nh q[0];\ncx q[0], q[1];\nmeasure q -> c;\n",
    "status": "Waiting",
    "updated_time": "2024-07-25T14:43:48.041348",
    "v_exec_shots": 0
  }
}
```

You can use the following command to submit a task:
```bash
# submit a task with code in examples/bell.qasm, the shots is 3000, the depth is 4, and the qubits is 2
cargo run -- -m emulate -f examples/bell.qasm -s 3000 -d 4 -q 2
```

### Get the result of a task
You can use `id` to get the result of the task. You can get the `id` from the `submit-task` command. For example:
```bash
{
  "task": {
    "created_time": "2024-07-25T14:43:48.041348",
    "depth": 4,
    "id": "2de9ab58-7a4a-44de-837d-280d1d50dd89",
    "qubits": 2,
    "result": "{\n  \"Memory\": {\n    \"00\": 1515,\n    \"11\": 1485\n  }\n}",
    "shots": 3000,
    "source": "OPENQASM 2.0;\ninclude \"qelib1.inc\";\nqreg q[2];\ncreg c[2];\nh q[0];\ncx q[0], q[1];\nmeasure q -> c;\n",
    "status": "Succeeded",
    "updated_time": "2024-07-25T14:43:52.752086"
  }
}
```

You can use the following command to get the result of the task:
```bash
cargo run -- -m get-task --task-id 2de9ab58-7a4a-44de-837d-280d1d50dd89
```