#!/usr/bin/env node

import "source-map-support/register";
import * as cdk from "@aws-cdk/core";
import { ApiStack } from "./api.stack";

type IAMPocAppProperties = {
  account: string;
};

class IAMPocApp extends cdk.Construct {
  constructor(
    scope: cdk.Construct,
    id: string,
    properties: IAMPocAppProperties
  ) {
    super(scope, id);
    new ApiStack(this, `IAMPocApp`, {
      env: {
        account: properties.account,
        region: "ap-southeast-2",
      },
    });
  }
}

const app = new cdk.App();

new IAMPocApp(app, "Dev", {
  account: "067289113644",
});
