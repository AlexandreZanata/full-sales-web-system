/** Maps report API error codes to user-facing messages (API-CONTRACT). */
export function reportActionErrorMessage(code: string): string {
  switch (code) {
    case 'SIGNING_KEY_UNAVAILABLE':
      return 'Report signing is temporarily unavailable. Contact an administrator.';
    case 'VALIDATION_ERROR':
      return 'Check the report type, period, and scope fields.';
    default:
      return 'Unable to generate the report. Please try again.';
  }
}
