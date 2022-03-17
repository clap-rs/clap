const completion: Fig.Spec = {
  name: "my-app",
  description: "",
  options: [
    {
      name: "--choice",
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
      args: {
        name: "unknown",
        isOptional: true,
      },
    },
    {
      name: "--other",
      args: {
        name: "other",
        isOptional: true,
      },
    },
    {
      name: ["-p", "--path"],
      args: {
        name: "path",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: ["-f", "--file"],
      args: {
        name: "file",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: ["-d", "--dir"],
      args: {
        name: "dir",
        isOptional: true,
        template: "folders",
      },
    },
    {
      name: ["-e", "--exe"],
      args: {
        name: "exe",
        isOptional: true,
        template: "filepaths",
      },
    },
    {
      name: "--cmd-name",
      args: {
        name: "cmd_name",
        isOptional: true,
        isCommand: true,
      },
    },
    {
      name: ["-c", "--cmd"],
      args: {
        name: "cmd",
        isOptional: true,
        isCommand: true,
      },
    },
    {
      name: ["-u", "--user"],
      args: {
        name: "user",
        isOptional: true,
      },
    },
    {
      name: ["-h", "--host"],
      args: {
        name: "host",
        isOptional: true,
      },
    },
    {
      name: "--url",
      args: {
        name: "url",
        isOptional: true,
      },
    },
    {
      name: "--email",
      args: {
        name: "email",
        isOptional: true,
      },
    },
    {
      name: "--help",
      description: "Print help information",
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
