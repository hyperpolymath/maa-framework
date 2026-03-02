// AuditTrail.res - DOM annotation for debugging
// Injects data-audit paths into the DOM

@val @scope("document")
external querySelectorAll: string => array<Dom.element> = "querySelectorAll"

@val @scope("document")
external createElement: string => Dom.element = "createElement"

@set external setTextContent: (Dom.element, string) => unit = "textContent"
@set external setCssText: ({..}, string) => unit = "cssText"
@get external getStyle: Dom.element => {..} = "style"
@get external getDataset: Dom.element => {..} = "dataset"
@get external getAudit: {..} => Js.nullable<string> = "audit"
@send external appendChild: (Dom.element, Dom.element) => unit = "appendChild"
@set external setPosition: ({..}, string) => unit = "position"

let annotateAudits = () => {
  let elements = querySelectorAll("[data-audit]")

  elements->Array.forEach(el => {
    let dataset = el->getDataset
    let auditValue = dataset->getAudit

    switch auditValue->Js.Nullable.toOption {
    | Some(audit) => {
        let tag = createElement("span")
        tag->setTextContent(audit)

        let tagStyle = tag->getStyle
        tagStyle->setCssText(
          "position:absolute; top:0; right:0; font-size:0.6rem; opacity:0.3;",
        )

        let elStyle = el->getStyle
        elStyle->setPosition("relative")

        el->appendChild(tag)
      }
    | None => ()
    }
  })
}

// Initialize on DOMContentLoaded
@val @scope("window")
external addEventListener: (string, unit => unit) => unit = "addEventListener"

let () = addEventListener("DOMContentLoaded", annotateAudits)
