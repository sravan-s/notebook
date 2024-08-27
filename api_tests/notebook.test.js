import { describe, expect, test } from 'vitest'
import { env } from './env';

const notebook_body = {
  "name": "my_notebook",
  "description": "my description",
  "dependencies": "a:version; b:version; c:version",
  "secrets": "a: val; b: val; c: val"
};

// I will just do a large test in this, is okay if it fails
describe("Notebook", () => {
  test('should create, fetch and delete a notebook', async () => {
    // create_notebook
    let request = await fetch(`${env.API}/notebook`, {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(notebook_body),
    });
    let notebook = await request.json();
    expect(notebook.id).toBeTypeOf("number");
    expect(notebook.name).toBeTypeOf("string");
    expect(notebook.description).toBeTypeOf("string");
    expect(notebook.updated_at).toBeTypeOf("number");

    // fetch notebook with ID
    let request_get = await fetch(`${env.API}/notebook/${notebook.id}`);
    let notebook_get = await request_get.json();
    expect(notebook_get.id).toBeTypeOf("number");
    expect(notebook_get.name).toBeTypeOf("string");
    expect(notebook_get.description).toBeTypeOf("string");
    expect(notebook_get.updated_at).toBeTypeOf("number");

    // fetch delete notebook
    let request_delete = await fetch(`${env.API}/notebook/${notebook.id}`, {
      method: "DELETE",
    });
    let notebook_delete = await request_delete.status;
    expect(notebook_delete).toEqual(200);
  });
})
