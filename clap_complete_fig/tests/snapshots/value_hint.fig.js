const completion: Fig.Spec = {
  name: "my-app",
  description: "",
  options: [
    {
      name: "--choice",
      isRepeatable: true,
      args: {
        name: "choice",
        isOptional: true,
        suggestions: [
          "bash",
          "fish",
          "zsh",
        ],
      },
    },
    {
      name: "--unknown",
      isRepeatable: true,
      args: {
        name: "unknown",
        isOptional: true,
      },
    },
    {
      name: "--other",
      isRepeatable: true,
      args: {
        name: "other",
        isOptional: true,
      },
    },
    {
      name: ["-p", "--path"],
      isRepeatable: true,
      args: {
        name: "path",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: ["-f", "--file"],
      isRepeatable: true,
      args: {
        name: "file",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: ["-d", "--dir"],
      isRepeatable: true,
      args: {
        name: "dir",
        isOptional: true,
        template: "folders",
      },
    },
    {
      name: ["-e", "--exe"],
      isRepeatable: true,
      args: {
        name: "exe",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: "--cmd-name",
      isRepeatable: true,
      args: {
        name: "cmd_name",
        isOptional: true,
        isCommand: true,
      },
    },
    {
      name: ["-c", "--cmd"],
      isRepeatable: true,
      args: {
        name: "cmd",
        isOptional: true,
        isCommand: true,
      },
    },
    {
      name: ["-u", "--user"],
      isRepeatable: true,
      args: {
        name: "user",
        isOptional: true,
      },
    },
    {
      name: ["-H", "--host"],
      isRepeatable: true,
      args: {
        name: "host",
        isOptional: true,
      },
    },
    {
      name: "--url",
      isRepeatable: true,
      args: {
        name: "url",
        isOptional: true,
      },
    },
    {
      name: "--email",
      isRepeatable: true,
      args: {
        name: "email",
        isOptional: true,
      },
    },
    {
      name: ["-h", "--help"],
      description: "Print help",
    },
  ],
  args: {
    name: "command_with_args",
    isVariadic: true,
    isOptional: true,
    isCommand: true,
  },
};

export default completion;
