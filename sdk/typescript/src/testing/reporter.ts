import { Runner, reporters } from "mocha";

const bold = "\x1b[1m";
const { color } = reporters.Base;
const { EVENT_RUN_BEGIN, EVENT_RUN_END, EVENT_SUITE_BEGIN, EVENT_SUITE_END, EVENT_TEST_PASS, EVENT_TEST_FAIL } =
  Runner.constants;

export class Reporter extends reporters.Base {
  constructor(runner: Runner) {
    super(runner);

    this.listen(runner);
  }

  listen(runner: Runner) {
    {
      runner
        .once(EVENT_RUN_BEGIN, () => this.log())
        .on(EVENT_SUITE_BEGIN, (suite) => {
          this.log("%s%s", bold, suite.title);
        })
        .on(EVENT_SUITE_END, () => this.log())
        .on(EVENT_TEST_PASS, (test) => {
          this.log("%s  [ %s    ] %s", bold, color("green", "PASS"), test.title);
        })
        .on(EVENT_TEST_FAIL, (test) => {
          this.log("%s  [ %s    ] %s", bold, color("fail", "FAIL"), test.title);
        })
        .once(EVENT_RUN_END, () => {
          const { passes, failures, tests } = this.stats;
          const output = "Test result: %s. Total tests: %d; passed: %d; failed: %d";

          if (failures != 0) {
            this.log("Test failures: \n");

            for (let i = 0; i < this.failures.length; i++) {
              const { err, title } = this.failures[i];

              this.log("%s  %s", bold, title);
              this.log("    %s", color("error message", `${err?.name}: ${err?.message}`));
              this.log("    %s\n", color("error stack", err?.stack!));
            }

            this.log(output, color("fail", `${bold}FAILED`), tests, passes, failures);
          } else {
            this.log(output, color("green", `${bold}OK`), tests, passes, failures);
          }
        });
    }
  }

  get log() {
    return console.log;
  }
}
