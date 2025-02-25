// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
/* eslint-env node */
"use strict";

async function test(context, commands) {
  // This violates all sorts of abstraction boundaries, but I don't see supported APIs for "just
  // waiting" nor for allowing navigation scripts to produce measurements.
  await commands.measure.start();
  await commands.measure.browser.wait(commands.measure.pageCompleteCheck);
  await commands.measure.stop();

  const browserScripts = commands.measure.result[0].browserScripts;

  const processLaunchToNavStart =
    browserScripts.pageinfo.navigationStartTime -
    browserScripts.browser.processStartTime;

  browserScripts.pageinfo.processLaunchToNavStart = processLaunchToNavStart;
  console.log("processLaunchToNavStart: " + processLaunchToNavStart);

  return true;
}

module.exports = {
  test,
  owner: "Performance Team",
  name: "App-link",
  description: "Measures cold process applink time",
  longDescription: `
  This test launches the appropriate android app, simulating an app link
  workflow. The application is launched with the intent action
  android.intent.action.VIEW loading a trivially simple website. The reported
  metric is the time from process start to navigationStart, reported as processLaunchToNavStart
  `,
  usage: `
  ./mach perftest testing/performance/perftest_applink.js \
    --android-install-apk ~/fenix.v2.fennec-nightly.2020.04.22-arm32.apk \
    --hooks testing/performance/hooks_applink.py \
    --android-app-name org.mozilla.fennec_aurora \
    --perfherder-metrics processLaunchToNavStart
  `,
  supportedBrowsers: ["Fenix nightly", "Geckoview_example", "Fennec"],
  supportedPlatforms: ["Android"],
};
