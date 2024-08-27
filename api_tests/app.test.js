import { describe, expect, test } from "vitest";
import { env } from "./env";

describe("APP", () => {
  test("Should check API health", async () => {
    const request = await fetch(env.API,  {
      method: "GET",
    });
    const result = await request.text(); 
    expect(result).toEqual("Server running");
  });
})