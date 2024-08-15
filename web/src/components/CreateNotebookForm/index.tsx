import { useState } from "react";
import "./create-ntoebook-form.css"

type Props = {}

export default function index({}: Props) {
  let [secrets, setSecrets] = useState<Array<string>>([]);

  return (
    <div className="create-notebook-form">
      <form onSubmit={(e) => {
        e.stopPropagation();
        e.preventDefault();
      }}>
        <h3>Create Notebook</h3>
        <section className="main_inputs">
          <div>
            <label htmlFor="notebook_name">Name</label>
            <input name="notebook_name" required />
          </div>
          <div>
            <label htmlFor="notebook_description">Description</label>
            <textarea name="notebook_description" maxLength={80} required />
          </div>
        </section>
        <section className="secret_inputs">
          <h4>Secrets</h4>
          {
            secrets.map((s) => {
              return (
                <div className="secret" key={s}>
                  <input required name={`key__${s}`} placeholder="key" />
                  <input required name={`value__${s}`} placeholder="value" type="password" />
                  <button type="button" onClick={() => {
                    setSecrets(secrets => {
                      return secrets.filter(secrets => s !== secrets)
                    })
                  }}>delete</button>
                </div>
              );
            })
          }
          <div>
            <button onClick={() => {
              const id = crypto.randomUUID();
              setSecrets([
                ...secrets,
                id,
              ])
            }} type="button">add secret</button>
          </div>
        </section>
        {/* Maybe add resources */}
        <div>
          <button type="submit">Create</button>
        </div>
      </form>
    </div>
  )
}