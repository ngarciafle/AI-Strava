package com.example.mobile

import androidx.compose.runtime.mutableDoubleStateOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import android.os.SystemClock

class TrainingViewModel: ViewModel() {
    private val motorRust = Training()

    var timeInSeconds = mutableDoubleStateOf(0.0)
        private set

    private var startTime = 0L
    private var accumulatedTime = 0L
    private var timerJob: Job? = null

    private val _stats = MutableStateFlow<StatsTraining>(
        StatsTraining(
            distance = 0.0,
            elevationGain = 0.0,
            elevationLoss = 0.0,
            rithm = 0.0,
            time = 0.0,
            rithms = listOf()
        )
    )
    val stats = _stats.asStateFlow()

    // NOT WORKS CAUSE ITS CALLED WHENEVER IT WANTS
    fun startTimer() {
        if (timerJob?.isActive == true) return

        startTime = SystemClock.elapsedRealtime()

        timerJob = viewModelScope.launch {
            while (true) {
                val now = SystemClock.elapsedRealtime()
                val currentSessionTime = now - startTime

                timeInSeconds.doubleValue = (accumulatedTime + currentSessionTime) / 1000.0

                delay(100L)
            }
        }
    }

    fun pauseTimer() {
        timerJob?.cancel()

        if (startTime > 0) {
            accumulatedTime += SystemClock.elapsedRealtime() - startTime
            startTime = 0
        }


    }

    fun registerPoint(lat: Double, lon: Double, alt: Double) {

        val newStats = motorRust.registerNewPoint(lat, lon, alt, timeInSeconds.doubleValue)
        _stats.value = newStats
    }

    fun endTraining() {
        motorRust.endTraining()
    }
}