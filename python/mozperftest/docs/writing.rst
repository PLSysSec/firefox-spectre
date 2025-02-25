Writing a browsertime test
==========================

With the browsertime layer, performance scenarios are Node modules that
implement at least one async function that will be called by the framework once
the browser has started. The function gets a webdriver session and can interact
with the browser.

You can write complex, interactive scenarios to simulate a user journey,
and collect various metrics.

Full documentation is available `here <https://www.sitespeed.io/documentation/sitespeed.io/scripting/>`_

The mozilla-central repository has a few performance tests script in
`testing/performance` and more should be added in components in the future.

By convention, a performance test is prefixed with **perftest_** to be
recognized by the `perftest` command.

A performance test implements at least one async function published in node's
`module.exports` as `test`. The function receives two objects:

- **context**, which contains:

  - **options** - All the options sent from the CLI to Browsertime
  - **log** - an instance to the log system so you can log from your navigation script
  - **index** - the index of the runs, so you can keep track of which run you are currently on
  - **storageManager** - The Browsertime storage manager that can help you read/store files to disk
  - **selenium.webdriver** - The Selenium WebDriver public API object
  - **selenium.driver** - The instantiated version of the WebDriver driving the current version of the browser

- **command** provides API to interact with the browser. It's a wrapper
  around the selenium client `Full documentation here <https://www.sitespeed.io/documentation/sitespeed.io/scripting/#commands>`_


Below is an example of a test that visits the BBC homepage and clicks on a link.

.. sourcecode:: javascript

    "use strict";

    async function setUp(context) {
      context.log.info("setUp example!");
    }

    async function test(context, commands) {
        await commands.navigate("https://www.bbc.com/");

        // Wait for browser to settle
        await commands.wait.byTime(10000);

        // Start the measurement
        await commands.measure.start("pageload");

        // Click on the link and wait for page complete check to finish.
        await commands.click.byClassNameAndWait("block-link__overlay-link");

        // Stop and collect the measurement
        await commands.measure.stop();
    }

    async function tearDown(context) {
      context.log.info("tearDown example!");
    }

    module.exports = {
        setUp,
        test,
        tearDown,
        owner: "Performance Team",
        test_name: "BBC",
        description: "Measures pageload performance when clicking on a link from the bbc.com",
        supportedBrowsers: "Any",
        supportePlatforms: "Any",
    };


Besides the `test` function, scripts can implement a `setUp` and a `tearDown` function to run
some code before and after the test. Those functions will be called just once, whereas
the `test` function might be called several times (through the `iterations` option)

You must also provide metadata information about the test:

- **owner**: name of the owner (person or team)
- **name**: name of the test
- **description**: short description
- **longDescription**: longer description
- **usage**: options used to run the test
- **supportedBrowsers**: list of supported browsers (or "Any")
- **supportedPlatforms**: list of supported platforms (or "Any")


Hooks
-----

A Python module can be used to run functions during a run lifecycle. Available hooks are:

- **before_runs(env)** runs before the test is launched. Can be used to
  change the running environment.
- **after_runs(env)** runs after the test is done.
- **on_exception(env, layer, exception)** called on any exception. Provides the
  layer in which the exception occured, and the exception. If the hook returns `True`
  the exception is ignored and the test resumes. If the hook returns `False`, the
  exception is ignored and the test ends immediatly. The hook can also re-raise the
  exception or raise its own exception.

In the example below, the `before_runs` hook is setting the options on the fly,
so users don't have to provide them in the command line::

    from mozperftest.browser.browsertime import add_options

    url = "'https://www.example.com'"

    common_options = [("processStartTime", "true"),
                      ("firefox.disableBrowsertimeExtension", "true"),
                      ("firefox.android.intentArgument", "'-a'"),
                      ("firefox.android.intentArgument", "'android.intent.action.VIEW'"),
                      ("firefox.android.intentArgument", "'-d'"),
                      ("firefox.android.intentArgument", url)]


    def before_runs(env, **kw):
        add_options(env, common_options)


To use this hook module, it can be passed to the `--hooks` option::

    $  ./mach perftest --hooks hooks.py perftest_example.js


