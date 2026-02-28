import React from 'react';

interface PromptInputProps {
    value: string;
    onChange: (value: string) => void;
    onSubmit: () => void;
    loading: boolean;
}

export const PromptInput: React.FC<PromptInputProps> = ({ value, onChange, onSubmit, loading }) => {
    const charCount = value.length;
    const maxChars = 4096;

    return (
        <div className="card prompt-input-card">
            <div className="card-header">
                <h2>📝 PROMPT INPUT</h2>
            </div>
            <div className="card-body">
                <textarea
                    className="prompt-textarea"
                    placeholder="Enter your prompt here..."
                    value={value}
                    onChange={(e) => onChange(e.target.value)}
                    maxLength={maxChars}
                    disabled={loading}
                />
                <div className="input-footer">
                    <span className="char-count">
                        {charCount}/{maxChars} chars
                    </span>
                    <button
                        className="submit-btn"
                        onClick={onSubmit}
                        disabled={loading || charCount === 0}
                    >
                        {loading ? 'Analyzing...' : 'Analyze & Generate'}
                    </button>
                </div>
            </div>
        </div>
    );
};
