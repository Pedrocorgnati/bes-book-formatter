// analytics.ts — PostHog + Sentry bootstrap — DISABLED by default (LGPD/GDPR)
// Analytics events are only sent when the user explicitly opts in via preferences.

import { preferencesStore } from '$lib/stores/preferencesStore';

let analyticsEnabled = false;
let unsubscribe: (() => void) | null = null;

/**
 * Initialize analytics system.
 * Subscribes to the preferences store for analytics_opt_in changes.
 * When enabled: initialize PostHog and Sentry (placeholder for now).
 * When disabled: ensure no events are sent.
 */
export function initAnalytics(): void {
  // Avoid double-subscribing
  if (unsubscribe) return;

  unsubscribe = preferencesStore.subscribe((prefs) => {
    const wasEnabled = analyticsEnabled;
    analyticsEnabled = prefs.analyticsOptIn;

    if (analyticsEnabled && !wasEnabled) {
      // TODO: Initialize PostHog — posthog.init(...)
      // TODO: Initialize Sentry — Sentry.init(...)
      console.debug('[analytics] Analytics enabled by user opt-in');
    } else if (!analyticsEnabled && wasEnabled) {
      // TODO: Shutdown PostHog — posthog.opt_out_capturing()
      // TODO: Shutdown Sentry
      console.debug('[analytics] Analytics disabled by user');
    }
  });
}

/**
 * Track a named event with optional properties.
 * No-op when analytics is disabled.
 */
export function trackEvent(event: string, props?: Record<string, unknown>): void {
  if (!analyticsEnabled) return;
  // TODO: posthog.capture(event, props)
  console.debug('[analytics]', event, props);
}

/**
 * Programmatically enable or disable analytics.
 */
export function setAnalyticsEnabled(enabled: boolean): void {
  analyticsEnabled = enabled;
}
