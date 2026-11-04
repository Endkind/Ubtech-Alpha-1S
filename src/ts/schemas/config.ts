export type Config = {
    mac_address: string;
    default_delay: number;
    default_action_duration: number;
    python_sdk: string;
    workdir: string;
    venv: string;
};

export type ConfigConflict = {
    original: Config;
    recovery: Config;
};

export type ResolutionChoice = 'original' | 'recovery';

export type RepairMethod = 'converted' | 'default_fallback';

export type FieldRepair = {
    field: string;
    original_value: any;
    expected_type: string;
    reason: string;
    resulting_value: any;
    repair_method: RepairMethod;
    success: boolean;
};

export type RecoveryEventType = 'info' | 'success' | 'warning' | 'error';

export type RecoveryDetails = {
    affected_file?: string | null;
    error_cause?: string | null;
    recovery_action?: string | null;
    source?: string | null;
    result?: string | null;
};

export type RecoveryEvent = {
    event_type: RecoveryEventType;
    title: string;
    description: string;
    details?: RecoveryDetails | null;
};

export type ConfigRepairReport = {
    repairs: FieldRepair[];
    unrecoverable_error?: string | null;
    recovery_event?: RecoveryEvent | null;
};
