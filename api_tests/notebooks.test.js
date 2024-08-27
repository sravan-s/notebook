import { describe, expect, test } from 'vitest'
import { env } from './env';

describe("Notebooks", () => {
  test('should list notebooks', async () => {
    let request = await fetch(`${env.API}/notebooks`);
    let response = await request.json();
    response.map(r => {
      expect(r.description).toBeTypeOf("string");
      expect(r.id).toBeTypeOf("number");
      expect(r.name).toBeTypeOf("string");
      expect(r.updated_at).toBeTypeOf("number");
    });
  })
})

