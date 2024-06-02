/**
 * @license
 * Copyright 2019 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const t = globalThis,
  s =
    t.ShadowRoot &&
    (void 0 === t.ShadyCSS || t.ShadyCSS.nativeShadow) &&
    "adoptedStyleSheets" in Document.prototype &&
    "replace" in CSSStyleSheet.prototype,
  i = Symbol(),
  e = new WeakMap();
class o {
  constructor(t, s, e) {
    if (((this._$cssResult$ = !0), e !== i))
      throw Error(
        "CSSResult is not constructable. Use `unsafeCSS` or `css` instead."
      );
    (this.cssText = t), (this.t = s);
  }
  get styleSheet() {
    let t = this.i;
    const i = this.t;
    if (s && void 0 === t) {
      const s = void 0 !== i && 1 === i.length;
      s && (t = e.get(i)),
        void 0 === t &&
          ((this.i = t = new CSSStyleSheet()).replaceSync(this.cssText),
          s && e.set(i, t));
    }
    return t;
  }
  toString() {
    return this.cssText;
  }
}
const h = (t) => new o("string" == typeof t ? t : t + "", void 0, i),
  r = (t, ...s) => {
    const e =
      1 === t.length
        ? t[0]
        : s.reduce(
            (s, i, e) =>
              s +
              ((t) => {
                if (!0 === t._$cssResult$) return t.cssText;
                if ("number" == typeof t) return t;
                throw Error(
                  "Value passed to 'css' function must be a 'css' function result: " +
                    t +
                    ". Use 'unsafeCSS' to pass non-literal values, but take care to ensure page security."
                );
              })(i) +
              t[e + 1],
            t[0]
          );
    return new o(e, t, i);
  },
  n = (i, e) => {
    if (s)
      i.adoptedStyleSheets = e.map((t) =>
        t instanceof CSSStyleSheet ? t : t.styleSheet
      );
    else
      for (const s of e) {
        const e = document.createElement("style"),
          o = t.litNonce;
        void 0 !== o && e.setAttribute("nonce", o),
          (e.textContent = s.cssText),
          i.appendChild(e);
      }
  },
  c = s
    ? (t) => t
    : (t) =>
        t instanceof CSSStyleSheet
          ? ((t) => {
              let s = "";
              for (const i of t.cssRules) s += i.cssText;
              return h(s);
            })(t)
          : t,
  /**
   * @license
   * Copyright 2017 Google LLC
   * SPDX-License-Identifier: BSD-3-Clause
   */ {
    is: a,
    defineProperty: l,
    getOwnPropertyDescriptor: u,
    getOwnPropertyNames: d,
    getOwnPropertySymbols: f,
    getPrototypeOf: p,
  } = Object,
  v = globalThis,
  m = v.trustedTypes,
  y = m ? m.emptyScript : "",
  g = v.reactiveElementPolyfillSupport,
  _ = (t, s) => t,
  b = {
    toAttribute(t, s) {
      switch (s) {
        case Boolean:
          t = t ? y : null;
          break;
        case Object:
        case Array:
          t = null == t ? t : JSON.stringify(t);
      }
      return t;
    },
    fromAttribute(t, s) {
      let i = t;
      switch (s) {
        case Boolean:
          i = null !== t;
          break;
        case Number:
          i = null === t ? null : Number(t);
          break;
        case Object:
        case Array:
          try {
            i = JSON.parse(t);
          } catch (t) {
            i = null;
          }
      }
      return i;
    },
  },
  S = (t, s) => !a(t, s),
  w = { attribute: !0, type: String, converter: b, reflect: !1, hasChanged: S };
(Symbol.metadata ??= Symbol("metadata")),
  (v.litPropertyMetadata ??= new WeakMap());
class $ extends HTMLElement {
  static addInitializer(t) {
    this.o(), (this.l ??= []).push(t);
  }
  static get observedAttributes() {
    return this.finalize(), this.u && [...this.u.keys()];
  }
  static createProperty(t, s = w) {
    if (
      (s.state && (s.attribute = !1),
      this.o(),
      this.elementProperties.set(t, s),
      !s.noAccessor)
    ) {
      const i = Symbol(),
        e = this.getPropertyDescriptor(t, i, s);
      void 0 !== e && l(this.prototype, t, e);
    }
  }
  static getPropertyDescriptor(t, s, i) {
    const { get: e, set: o } = u(this.prototype, t) ?? {
      get() {
        return this[s];
      },
      set(t) {
        this[s] = t;
      },
    };
    return {
      get() {
        return e?.call(this);
      },
      set(s) {
        const h = e?.call(this);
        o.call(this, s), this.requestUpdate(t, h, i);
      },
      configurable: !0,
      enumerable: !0,
    };
  }
  static getPropertyOptions(t) {
    return this.elementProperties.get(t) ?? w;
  }
  static o() {
    if (this.hasOwnProperty(_("elementProperties"))) return;
    const t = p(this);
    t.finalize(),
      void 0 !== t.l && (this.l = [...t.l]),
      (this.elementProperties = new Map(t.elementProperties));
  }
  static finalize() {
    if (this.hasOwnProperty(_("finalized"))) return;
    if (
      ((this.finalized = !0), this.o(), this.hasOwnProperty(_("properties")))
    ) {
      const t = this.properties,
        s = [...d(t), ...f(t)];
      for (const i of s) this.createProperty(i, t[i]);
    }
    const t = this[Symbol.metadata];
    if (null !== t) {
      const s = litPropertyMetadata.get(t);
      if (void 0 !== s)
        for (const [t, i] of s) this.elementProperties.set(t, i);
    }
    this.u = new Map();
    for (const [t, s] of this.elementProperties) {
      const i = this.p(t, s);
      void 0 !== i && this.u.set(i, t);
    }
    this.elementStyles = this.finalizeStyles(this.styles);
  }
  static finalizeStyles(t) {
    const s = [];
    if (Array.isArray(t)) {
      const i = new Set(t.flat(1 / 0).reverse());
      for (const t of i) s.unshift(c(t));
    } else void 0 !== t && s.push(c(t));
    return s;
  }
  static p(t, s) {
    const i = s.attribute;
    return !1 === i
      ? void 0
      : "string" == typeof i
      ? i
      : "string" == typeof t
      ? t.toLowerCase()
      : void 0;
  }
  constructor() {
    super(),
      (this.v = void 0),
      (this.isUpdatePending = !1),
      (this.hasUpdated = !1),
      (this.m = null),
      this._();
  }
  _() {
    (this.S = new Promise((t) => (this.enableUpdating = t))),
      (this._$AL = new Map()),
      this.$(),
      this.requestUpdate(),
      this.constructor.l?.forEach((t) => t(this));
  }
  addController(t) {
    (this.P ??= new Set()).add(t),
      void 0 !== this.renderRoot && this.isConnected && t.hostConnected?.();
  }
  removeController(t) {
    this.P?.delete(t);
  }
  $() {
    const t = new Map(),
      s = this.constructor.elementProperties;
    for (const i of s.keys())
      this.hasOwnProperty(i) && (t.set(i, this[i]), delete this[i]);
    t.size > 0 && (this.v = t);
  }
  createRenderRoot() {
    const t =
      this.shadowRoot ?? this.attachShadow(this.constructor.shadowRootOptions);
    return n(t, this.constructor.elementStyles), t;
  }
  connectedCallback() {
    (this.renderRoot ??= this.createRenderRoot()),
      this.enableUpdating(!0),
      this.P?.forEach((t) => t.hostConnected?.());
  }
  enableUpdating(t) {}
  disconnectedCallback() {
    this.P?.forEach((t) => t.hostDisconnected?.());
  }
  attributeChangedCallback(t, s, i) {
    this._$AK(t, i);
  }
  C(t, s) {
    const i = this.constructor.elementProperties.get(t),
      e = this.constructor.p(t, i);
    if (void 0 !== e && !0 === i.reflect) {
      const o = (
        void 0 !== i.converter?.toAttribute ? i.converter : b
      ).toAttribute(s, i.type);
      (this.m = t),
        null == o ? this.removeAttribute(e) : this.setAttribute(e, o),
        (this.m = null);
    }
  }
  _$AK(t, s) {
    const i = this.constructor,
      e = i.u.get(t);
    if (void 0 !== e && this.m !== e) {
      const t = i.getPropertyOptions(e),
        o =
          "function" == typeof t.converter
            ? { fromAttribute: t.converter }
            : void 0 !== t.converter?.fromAttribute
            ? t.converter
            : b;
      (this.m = e), (this[e] = o.fromAttribute(s, t.type)), (this.m = null);
    }
  }
  requestUpdate(t, s, i) {
    if (void 0 !== t) {
      if (
        ((i ??= this.constructor.getPropertyOptions(t)),
        !(i.hasChanged ?? S)(this[t], s))
      )
        return;
      this.T(t, s, i);
    }
    !1 === this.isUpdatePending && (this.S = this.A());
  }
  T(t, s, i) {
    this._$AL.has(t) || this._$AL.set(t, s),
      !0 === i.reflect && this.m !== t && (this.M ??= new Set()).add(t);
  }
  async A() {
    this.isUpdatePending = !0;
    try {
      await this.S;
    } catch (t) {
      Promise.reject(t);
    }
    const t = this.scheduleUpdate();
    return null != t && (await t), !this.isUpdatePending;
  }
  scheduleUpdate() {
    return this.performUpdate();
  }
  performUpdate() {
    if (!this.isUpdatePending) return;
    if (!this.hasUpdated) {
      if (((this.renderRoot ??= this.createRenderRoot()), this.v)) {
        for (const [t, s] of this.v) this[t] = s;
        this.v = void 0;
      }
      const t = this.constructor.elementProperties;
      if (t.size > 0)
        for (const [s, i] of t)
          !0 !== i.wrapped ||
            this._$AL.has(s) ||
            void 0 === this[s] ||
            this.T(s, this[s], i);
    }
    let t = !1;
    const s = this._$AL;
    try {
      (t = this.shouldUpdate(s)),
        t
          ? (this.willUpdate(s),
            this.P?.forEach((t) => t.hostUpdate?.()),
            this.update(s))
          : this.k();
    } catch (s) {
      throw ((t = !1), this.k(), s);
    }
    t && this._$AE(s);
  }
  willUpdate(t) {}
  _$AE(t) {
    this.P?.forEach((t) => t.hostUpdated?.()),
      this.hasUpdated || ((this.hasUpdated = !0), this.firstUpdated(t)),
      this.updated(t);
  }
  k() {
    (this._$AL = new Map()), (this.isUpdatePending = !1);
  }
  get updateComplete() {
    return this.getUpdateComplete();
  }
  getUpdateComplete() {
    return this.S;
  }
  shouldUpdate(t) {
    return !0;
  }
  update(t) {
    (this.M &&= this.M.forEach((t) => this.C(t, this[t]))), this.k();
  }
  updated(t) {}
  firstUpdated(t) {}
}
($.elementStyles = []),
  ($.shadowRootOptions = { mode: "open" }),
  ($[_("elementProperties")] = new Map()),
  ($[_("finalized")] = new Map()),
  g?.({ ReactiveElement: $ }),
  (v.reactiveElementVersions ??= []).push("2.0.4");
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const P = globalThis,
  C = P.trustedTypes,
  T = C ? C.createPolicy("lit-html", { createHTML: (t) => t }) : void 0,
  x = "$lit$",
  A = `lit$${Math.random().toFixed(9).slice(2)}$`,
  M = "?" + A,
  k = `<${M}>`,
  E = document,
  U = () => E.createComment(""),
  N = (t) => null === t || ("object" != typeof t && "function" != typeof t),
  O = Array.isArray,
  R = (t) => O(t) || "function" == typeof t?.[Symbol.iterator],
  z = "[ \t\n\f\r]",
  V = /<(?:(!--|\/[^a-zA-Z])|(\/?[a-zA-Z][^>\s]*)|(\/?$))/g,
  L = /-->/g,
  I = />/g,
  j = RegExp(
    `>|${z}(?:([^\\s"'>=/]+)(${z}*=${z}*(?:[^ \t\n\f\r"'\`<>=]|("|')|))|$)`,
    "g"
  ),
  D = /'/g,
  H = /"/g,
  B = /^(?:script|style|textarea|title)$/i,
  W =
    (t) =>
    (s, ...i) => ({ _$litType$: t, strings: s, values: i }),
  q = W(1),
  J = W(2),
  Z = Symbol.for("lit-noChange"),
  F = Symbol.for("lit-nothing"),
  G = new WeakMap(),
  K = E.createTreeWalker(E, 129);
function Q(t, s) {
  if (!Array.isArray(t) || !t.hasOwnProperty("raw"))
    throw Error("invalid template strings array");
  return void 0 !== T ? T.createHTML(s) : s;
}
const X = (t, s) => {
  const i = t.length - 1,
    e = [];
  let o,
    h = 2 === s ? "<svg>" : "",
    r = V;
  for (let s = 0; s < i; s++) {
    const i = t[s];
    let n,
      c,
      a = -1,
      l = 0;
    for (; l < i.length && ((r.lastIndex = l), (c = r.exec(i)), null !== c); )
      (l = r.lastIndex),
        r === V
          ? "!--" === c[1]
            ? (r = L)
            : void 0 !== c[1]
            ? (r = I)
            : void 0 !== c[2]
            ? (B.test(c[2]) && (o = RegExp("</" + c[2], "g")), (r = j))
            : void 0 !== c[3] && (r = j)
          : r === j
          ? ">" === c[0]
            ? ((r = o ?? V), (a = -1))
            : void 0 === c[1]
            ? (a = -2)
            : ((a = r.lastIndex - c[2].length),
              (n = c[1]),
              (r = void 0 === c[3] ? j : '"' === c[3] ? H : D))
          : r === H || r === D
          ? (r = j)
          : r === L || r === I
          ? (r = V)
          : ((r = j), (o = void 0));
    const u = r === j && t[s + 1].startsWith("/>") ? " " : "";
    h +=
      r === V
        ? i + k
        : a >= 0
        ? (e.push(n), i.slice(0, a) + x + i.slice(a) + A + u)
        : i + A + (-2 === a ? s : u);
  }
  return [Q(t, h + (t[i] || "<?>") + (2 === s ? "</svg>" : "")), e];
};
class Y {
  constructor({ strings: t, _$litType$: s }, i) {
    let e;
    this.parts = [];
    let o = 0,
      h = 0;
    const r = t.length - 1,
      n = this.parts,
      [c, a] = X(t, s);
    if (
      ((this.el = Y.createElement(c, i)),
      (K.currentNode = this.el.content),
      2 === s)
    ) {
      const t = this.el.content.firstChild;
      t.replaceWith(...t.childNodes);
    }
    for (; null !== (e = K.nextNode()) && n.length < r; ) {
      if (1 === e.nodeType) {
        if (e.hasAttributes())
          for (const t of e.getAttributeNames())
            if (t.endsWith(x)) {
              const s = a[h++],
                i = e.getAttribute(t).split(A),
                r = /([.?@])?(.*)/.exec(s);
              n.push({
                type: 1,
                index: o,
                name: r[2],
                strings: i,
                ctor:
                  "." === r[1]
                    ? ot
                    : "?" === r[1]
                    ? ht
                    : "@" === r[1]
                    ? rt
                    : et,
              }),
                e.removeAttribute(t);
            } else
              t.startsWith(A) &&
                (n.push({ type: 6, index: o }), e.removeAttribute(t));
        if (B.test(e.tagName)) {
          const t = e.textContent.split(A),
            s = t.length - 1;
          if (s > 0) {
            e.textContent = C ? C.emptyScript : "";
            for (let i = 0; i < s; i++)
              e.append(t[i], U()),
                K.nextNode(),
                n.push({ type: 2, index: ++o });
            e.append(t[s], U());
          }
        }
      } else if (8 === e.nodeType)
        if (e.data === M) n.push({ type: 2, index: o });
        else {
          let t = -1;
          for (; -1 !== (t = e.data.indexOf(A, t + 1)); )
            n.push({ type: 7, index: o }), (t += A.length - 1);
        }
      o++;
    }
  }
  static createElement(t, s) {
    const i = E.createElement("template");
    return (i.innerHTML = t), i;
  }
}
function tt(t, s, i = t, e) {
  if (s === Z) return s;
  let o = void 0 !== e ? i.U?.[e] : i.N;
  const h = N(s) ? void 0 : s._$litDirective$;
  return (
    o?.constructor !== h &&
      (o?._$AO?.(!1),
      void 0 === h ? (o = void 0) : ((o = new h(t)), o._$AT(t, i, e)),
      void 0 !== e ? ((i.U ??= [])[e] = o) : (i.N = o)),
    void 0 !== o && (s = tt(t, o._$AS(t, s.values), o, e)),
    s
  );
}
class st {
  constructor(t, s) {
    (this._$AV = []), (this._$AN = void 0), (this._$AD = t), (this._$AM = s);
  }
  get parentNode() {
    return this._$AM.parentNode;
  }
  get _$AU() {
    return this._$AM._$AU;
  }
  O(t) {
    const {
        el: { content: s },
        parts: i,
      } = this._$AD,
      e = (t?.creationScope ?? E).importNode(s, !0);
    K.currentNode = e;
    let o = K.nextNode(),
      h = 0,
      r = 0,
      n = i[0];
    for (; void 0 !== n; ) {
      if (h === n.index) {
        let s;
        2 === n.type
          ? (s = new it(o, o.nextSibling, this, t))
          : 1 === n.type
          ? (s = new n.ctor(o, n.name, n.strings, this, t))
          : 6 === n.type && (s = new nt(o, this, t)),
          this._$AV.push(s),
          (n = i[++r]);
      }
      h !== n?.index && ((o = K.nextNode()), h++);
    }
    return (K.currentNode = E), e;
  }
  R(t) {
    let s = 0;
    for (const i of this._$AV)
      void 0 !== i &&
        (void 0 !== i.strings
          ? (i._$AI(t, i, s), (s += i.strings.length - 2))
          : i._$AI(t[s])),
        s++;
  }
}
class it {
  get _$AU() {
    return this._$AM?._$AU ?? this.V;
  }
  constructor(t, s, i, e) {
    (this.type = 2),
      (this._$AH = F),
      (this._$AN = void 0),
      (this._$AA = t),
      (this._$AB = s),
      (this._$AM = i),
      (this.options = e),
      (this.V = e?.isConnected ?? !0);
  }
  get parentNode() {
    let t = this._$AA.parentNode;
    const s = this._$AM;
    return void 0 !== s && 11 === t?.nodeType && (t = s.parentNode), t;
  }
  get startNode() {
    return this._$AA;
  }
  get endNode() {
    return this._$AB;
  }
  _$AI(t, s = this) {
    (t = tt(this, t, s)),
      N(t)
        ? t === F || null == t || "" === t
          ? (this._$AH !== F && this._$AR(), (this._$AH = F))
          : t !== this._$AH && t !== Z && this.L(t)
        : void 0 !== t._$litType$
        ? this.I(t)
        : void 0 !== t.nodeType
        ? this.j(t)
        : R(t)
        ? this.D(t)
        : this.L(t);
  }
  H(t) {
    return this._$AA.parentNode.insertBefore(t, this._$AB);
  }
  j(t) {
    this._$AH !== t && (this._$AR(), (this._$AH = this.H(t)));
  }
  L(t) {
    this._$AH !== F && N(this._$AH)
      ? (this._$AA.nextSibling.data = t)
      : this.j(E.createTextNode(t)),
      (this._$AH = t);
  }
  I(t) {
    const { values: s, _$litType$: i } = t,
      e =
        "number" == typeof i
          ? this._$AC(t)
          : (void 0 === i.el &&
              (i.el = Y.createElement(Q(i.h, i.h[0]), this.options)),
            i);
    if (this._$AH?._$AD === e) this._$AH.R(s);
    else {
      const t = new st(e, this),
        i = t.O(this.options);
      t.R(s), this.j(i), (this._$AH = t);
    }
  }
  _$AC(t) {
    let s = G.get(t.strings);
    return void 0 === s && G.set(t.strings, (s = new Y(t))), s;
  }
  D(t) {
    O(this._$AH) || ((this._$AH = []), this._$AR());
    const s = this._$AH;
    let i,
      e = 0;
    for (const o of t)
      e === s.length
        ? s.push((i = new it(this.H(U()), this.H(U()), this, this.options)))
        : (i = s[e]),
        i._$AI(o),
        e++;
    e < s.length && (this._$AR(i && i._$AB.nextSibling, e), (s.length = e));
  }
  _$AR(t = this._$AA.nextSibling, s) {
    for (this._$AP?.(!1, !0, s); t && t !== this._$AB; ) {
      const s = t.nextSibling;
      t.remove(), (t = s);
    }
  }
  setConnected(t) {
    void 0 === this._$AM && ((this.V = t), this._$AP?.(t));
  }
}
class et {
  get tagName() {
    return this.element.tagName;
  }
  get _$AU() {
    return this._$AM._$AU;
  }
  constructor(t, s, i, e, o) {
    (this.type = 1),
      (this._$AH = F),
      (this._$AN = void 0),
      (this.element = t),
      (this.name = s),
      (this._$AM = e),
      (this.options = o),
      i.length > 2 || "" !== i[0] || "" !== i[1]
        ? ((this._$AH = Array(i.length - 1).fill(new String())),
          (this.strings = i))
        : (this._$AH = F);
  }
  _$AI(t, s = this, i, e) {
    const o = this.strings;
    let h = !1;
    if (void 0 === o)
      (t = tt(this, t, s, 0)),
        (h = !N(t) || (t !== this._$AH && t !== Z)),
        h && (this._$AH = t);
    else {
      const e = t;
      let r, n;
      for (t = o[0], r = 0; r < o.length - 1; r++)
        (n = tt(this, e[i + r], s, r)),
          n === Z && (n = this._$AH[r]),
          (h ||= !N(n) || n !== this._$AH[r]),
          n === F ? (t = F) : t !== F && (t += (n ?? "") + o[r + 1]),
          (this._$AH[r] = n);
    }
    h && !e && this.B(t);
  }
  B(t) {
    t === F
      ? this.element.removeAttribute(this.name)
      : this.element.setAttribute(this.name, t ?? "");
  }
}
class ot extends et {
  constructor() {
    super(...arguments), (this.type = 3);
  }
  B(t) {
    this.element[this.name] = t === F ? void 0 : t;
  }
}
class ht extends et {
  constructor() {
    super(...arguments), (this.type = 4);
  }
  B(t) {
    this.element.toggleAttribute(this.name, !!t && t !== F);
  }
}
class rt extends et {
  constructor(t, s, i, e, o) {
    super(t, s, i, e, o), (this.type = 5);
  }
  _$AI(t, s = this) {
    if ((t = tt(this, t, s, 0) ?? F) === Z) return;
    const i = this._$AH,
      e =
        (t === F && i !== F) ||
        t.capture !== i.capture ||
        t.once !== i.once ||
        t.passive !== i.passive,
      o = t !== F && (i === F || e);
    e && this.element.removeEventListener(this.name, this, i),
      o && this.element.addEventListener(this.name, this, t),
      (this._$AH = t);
  }
  handleEvent(t) {
    "function" == typeof this._$AH
      ? this._$AH.call(this.options?.host ?? this.element, t)
      : this._$AH.handleEvent(t);
  }
}
class nt {
  constructor(t, s, i) {
    (this.element = t),
      (this.type = 6),
      (this._$AN = void 0),
      (this._$AM = s),
      (this.options = i);
  }
  get _$AU() {
    return this._$AM._$AU;
  }
  _$AI(t) {
    tt(this, t);
  }
}
const ct = {
    W: x,
    q: A,
    J: M,
    Z: 1,
    F: X,
    G: st,
    K: R,
    X: tt,
    Y: it,
    tt: et,
    st: ht,
    it: rt,
    et: ot,
    ot: nt,
  },
  at = P.litHtmlPolyfillSupport;
at?.(Y, it), (P.litHtmlVersions ??= []).push("3.1.3");
const lt = (t, s, i) => {
  const e = i?.renderBefore ?? s;
  let o = e._$litPart$;
  if (void 0 === o) {
    const t = i?.renderBefore ?? null;
    e._$litPart$ = o = new it(s.insertBefore(U(), t), t, void 0, i ?? {});
  }
  return o._$AI(t), o;
};
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */ class ut extends $ {
  constructor() {
    super(...arguments),
      (this.renderOptions = { host: this }),
      (this.ht = void 0);
  }
  createRenderRoot() {
    const t = super.createRenderRoot();
    return (this.renderOptions.renderBefore ??= t.firstChild), t;
  }
  update(t) {
    const s = this.render();
    this.hasUpdated || (this.renderOptions.isConnected = this.isConnected),
      super.update(t),
      (this.ht = lt(s, this.renderRoot, this.renderOptions));
  }
  connectedCallback() {
    super.connectedCallback(), this.ht?.setConnected(!0);
  }
  disconnectedCallback() {
    super.disconnectedCallback(), this.ht?.setConnected(!1);
  }
  render() {
    return Z;
  }
}
(ut._$litElement$ = !0),
  (ut[("finalized", "finalized")] = !0),
  globalThis.litElementHydrateSupport?.({ LitElement: ut });
const dt = globalThis.litElementPolyfillSupport;
dt?.({ LitElement: ut });
const ft = {
  _$AK: (t, s, i) => {
    t._$AK(s, i);
  },
  _$AL: (t) => t._$AL,
};
(globalThis.litElementVersions ??= []).push("4.0.5");
/**
 * @license
 * Copyright 2022 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const pt = !1;
export {
  o as CSSResult,
  ut as LitElement,
  $ as ReactiveElement,
  ft as _$LE,
  ct as _$LH,
  n as adoptStyles,
  r as css,
  b as defaultConverter,
  c as getCompatibleStyle,
  q as html,
  pt as isServer,
  Z as noChange,
  S as notEqual,
  F as nothing,
  lt as render,
  s as supportsAdoptingStyleSheets,
  J as svg,
  h as unsafeCSS,
};
//# sourceMappingURL=lit-core.min.js.map
