// @ts-nocheck
// analytics.ts — PostHog + Sentry bootstrap — DISABLED by default (LGPD/GDPR)
// Analytics events are only sent when the user explicitly opts in via preferences.
function stryNS_9fa48() {
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
import { preferencesStore } from '$lib/stores/preferencesStore';
let analyticsEnabled = stryMutAct_9fa48("542") ? true : (stryCov_9fa48("542"), false);
let unsubscribe: (() => void) | null = null;

/**
 * Initialize analytics system.
 * Subscribes to the preferences store for analytics_opt_in changes.
 * When enabled: initialize PostHog and Sentry (placeholder for now).
 * When disabled: ensure no events are sent.
 */
export function initAnalytics(): void {
  if (stryMutAct_9fa48("543")) {
    {}
  } else {
    stryCov_9fa48("543");
    // Avoid double-subscribing
    if (stryMutAct_9fa48("545") ? false : stryMutAct_9fa48("544") ? true : (stryCov_9fa48("544", "545"), unsubscribe)) return;
    unsubscribe = preferencesStore.subscribe(prefs => {
      if (stryMutAct_9fa48("546")) {
        {}
      } else {
        stryCov_9fa48("546");
        const wasEnabled = analyticsEnabled;
        analyticsEnabled = prefs.analyticsOptIn;
        if (stryMutAct_9fa48("549") ? analyticsEnabled || !wasEnabled : stryMutAct_9fa48("548") ? false : stryMutAct_9fa48("547") ? true : (stryCov_9fa48("547", "548", "549"), analyticsEnabled && (stryMutAct_9fa48("550") ? wasEnabled : (stryCov_9fa48("550"), !wasEnabled)))) {
          if (stryMutAct_9fa48("551")) {
            {}
          } else {
            stryCov_9fa48("551");
            // TODO: Initialize PostHog — posthog.init(...)
            // TODO: Initialize Sentry — Sentry.init(...)
            console.debug(stryMutAct_9fa48("552") ? "" : (stryCov_9fa48("552"), '[analytics] Analytics enabled by user opt-in'));
          }
        } else if (stryMutAct_9fa48("555") ? !analyticsEnabled || wasEnabled : stryMutAct_9fa48("554") ? false : stryMutAct_9fa48("553") ? true : (stryCov_9fa48("553", "554", "555"), (stryMutAct_9fa48("556") ? analyticsEnabled : (stryCov_9fa48("556"), !analyticsEnabled)) && wasEnabled)) {
          if (stryMutAct_9fa48("557")) {
            {}
          } else {
            stryCov_9fa48("557");
            // TODO: Shutdown PostHog — posthog.opt_out_capturing()
            // TODO: Shutdown Sentry
            console.debug(stryMutAct_9fa48("558") ? "" : (stryCov_9fa48("558"), '[analytics] Analytics disabled by user'));
          }
        }
      }
    });
  }
}

/**
 * Track a named event with optional properties.
 * No-op when analytics is disabled.
 */
export function trackEvent(event: string, props?: Record<string, unknown>): void {
  if (stryMutAct_9fa48("559")) {
    {}
  } else {
    stryCov_9fa48("559");
    if (stryMutAct_9fa48("562") ? false : stryMutAct_9fa48("561") ? true : stryMutAct_9fa48("560") ? analyticsEnabled : (stryCov_9fa48("560", "561", "562"), !analyticsEnabled)) return;
    // TODO: posthog.capture(event, props)
    console.debug(stryMutAct_9fa48("563") ? "" : (stryCov_9fa48("563"), '[analytics]'), event, props);
  }
}

/**
 * Programmatically enable or disable analytics.
 */
export function setAnalyticsEnabled(enabled: boolean): void {
  if (stryMutAct_9fa48("564")) {
    {}
  } else {
    stryCov_9fa48("564");
    analyticsEnabled = enabled;
  }
}