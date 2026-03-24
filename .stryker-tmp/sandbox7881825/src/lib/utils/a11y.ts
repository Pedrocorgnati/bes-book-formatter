/**
 * WCAG AA accessibility helpers
 */
// @ts-nocheck


/**
 * Creates a focus trap within a container element.
 * Returns a cleanup function.
 */function stryNS_9fa48() {
  var g = typeof globalThis === 'object' && globalThis && globalThis.Math === Math && globalThis || new Function("return this")();
  var ns = g.__stryker__ || (g.__stryker__ = {});
  if (ns.activeMutant === undefined && g.process && g.process.env && g.process.env.__STRYKER_ACTIVE_MUTANT__) {
    ns.activeMutant = g.process.env.__STRYKER_ACTIVE_MUTANT__;
  }
  function retrieveNS() {
    return ns;
  }
  stryNS_9fa48 = retrieveNS;
  return retrieveNS();
}
stryNS_9fa48();
function stryCov_9fa48() {
  var ns = stryNS_9fa48();
  var cov = ns.mutantCoverage || (ns.mutantCoverage = {
    static: {},
    perTest: {}
  });
  function cover() {
    var c = cov.static;
    if (ns.currentTestId) {
      c = cov.perTest[ns.currentTestId] = cov.perTest[ns.currentTestId] || {};
    }
    var a = arguments;
    for (var i = 0; i < a.length; i++) {
      c[a[i]] = (c[a[i]] || 0) + 1;
    }
  }
  stryCov_9fa48 = cover;
  cover.apply(null, arguments);
}
function stryMutAct_9fa48(id) {
  var ns = stryNS_9fa48();
  function isActive(id) {
    if (ns.activeMutant === id) {
      if (ns.hitCount !== void 0 && ++ns.hitCount > ns.hitLimit) {
        throw new Error('Stryker: Hit count limit reached (' + ns.hitCount + ')');
      }
      return true;
    }
    return false;
  }
  stryMutAct_9fa48 = isActive;
  return isActive(id);
}
export function trapFocus(container: HTMLElement): () => void {
  if (stryMutAct_9fa48("503")) {
    {}
  } else {
    stryCov_9fa48("503");
    const focusableSelector = stryMutAct_9fa48("504") ? "" : (stryCov_9fa48("504"), 'a[href], button:not([disabled]), textarea, input, select, [tabindex]:not([tabindex="-1"])');
    const handler = (e: KeyboardEvent) => {
      if (stryMutAct_9fa48("505")) {
        {}
      } else {
        stryCov_9fa48("505");
        if (stryMutAct_9fa48("508") ? e.key === 'Tab' : stryMutAct_9fa48("507") ? false : stryMutAct_9fa48("506") ? true : (stryCov_9fa48("506", "507", "508"), e.key !== (stryMutAct_9fa48("509") ? "" : (stryCov_9fa48("509"), 'Tab')))) return;
        const focusable = Array.from(container.querySelectorAll<HTMLElement>(focusableSelector));
        if (stryMutAct_9fa48("512") ? focusable.length !== 0 : stryMutAct_9fa48("511") ? false : stryMutAct_9fa48("510") ? true : (stryCov_9fa48("510", "511", "512"), focusable.length === 0)) return;
        const first = focusable[0];
        const last = focusable[stryMutAct_9fa48("513") ? focusable.length + 1 : (stryCov_9fa48("513"), focusable.length - 1)];
        if (stryMutAct_9fa48("515") ? false : stryMutAct_9fa48("514") ? true : (stryCov_9fa48("514", "515"), e.shiftKey)) {
          if (stryMutAct_9fa48("516")) {
            {}
          } else {
            stryCov_9fa48("516");
            if (stryMutAct_9fa48("519") ? document.activeElement !== first : stryMutAct_9fa48("518") ? false : stryMutAct_9fa48("517") ? true : (stryCov_9fa48("517", "518", "519"), document.activeElement === first)) {
              if (stryMutAct_9fa48("520")) {
                {}
              } else {
                stryCov_9fa48("520");
                e.preventDefault();
                last.focus();
              }
            }
          }
        } else {
          if (stryMutAct_9fa48("521")) {
            {}
          } else {
            stryCov_9fa48("521");
            if (stryMutAct_9fa48("524") ? document.activeElement !== last : stryMutAct_9fa48("523") ? false : stryMutAct_9fa48("522") ? true : (stryCov_9fa48("522", "523", "524"), document.activeElement === last)) {
              if (stryMutAct_9fa48("525")) {
                {}
              } else {
                stryCov_9fa48("525");
                e.preventDefault();
                first.focus();
              }
            }
          }
        }
      }
    };
    container.addEventListener(stryMutAct_9fa48("526") ? "" : (stryCov_9fa48("526"), 'keydown'), handler);
    return stryMutAct_9fa48("527") ? () => undefined : (stryCov_9fa48("527"), () => container.removeEventListener(stryMutAct_9fa48("528") ? "" : (stryCov_9fa48("528"), 'keydown'), handler));
  }
}

/**
 * Announces a message to screen readers via a live region.
 */
export function announceToScreenReader(message: string, priority: 'polite' | 'assertive' = stryMutAct_9fa48("529") ? "" : (stryCov_9fa48("529"), 'polite')): void {
  if (stryMutAct_9fa48("530")) {
    {}
  } else {
    stryCov_9fa48("530");
    const el = document.createElement(stryMutAct_9fa48("531") ? "" : (stryCov_9fa48("531"), 'div'));
    el.setAttribute(stryMutAct_9fa48("532") ? "" : (stryCov_9fa48("532"), 'aria-live'), priority);
    el.setAttribute(stryMutAct_9fa48("533") ? "" : (stryCov_9fa48("533"), 'aria-atomic'), stryMutAct_9fa48("534") ? "" : (stryCov_9fa48("534"), 'true'));
    el.setAttribute(stryMutAct_9fa48("535") ? "" : (stryCov_9fa48("535"), 'class'), stryMutAct_9fa48("536") ? "" : (stryCov_9fa48("536"), 'sr-only'));
    el.style.cssText = stryMutAct_9fa48("537") ? "" : (stryCov_9fa48("537"), 'position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0,0,0,0);');
    document.body.appendChild(el);

    // Delay to ensure screen reader picks up the change
    requestAnimationFrame(() => {
      if (stryMutAct_9fa48("538")) {
        {}
      } else {
        stryCov_9fa48("538");
        el.textContent = message;
        setTimeout(stryMutAct_9fa48("539") ? () => undefined : (stryCov_9fa48("539"), () => el.remove()), 3000);
      }
    });
  }
}

/**
 * Checks if user prefers reduced motion.
 */
export function prefersReducedMotion(): boolean {
  if (stryMutAct_9fa48("540")) {
    {}
  } else {
    stryCov_9fa48("540");
    return window.matchMedia(stryMutAct_9fa48("541") ? "" : (stryCov_9fa48("541"), '(prefers-reduced-motion: reduce)')).matches;
  }
}