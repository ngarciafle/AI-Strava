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
    var timeLapInSeconds = mutableDoubleStateOf(0.0)
        private set

    private var startTime = 0L
    private var lapStartTime = 0L
    private var lapStartDistance = 0.0
    private var accumulatedTime = 0L
    private var accumulatedTimeLap = 0L
    private var timerJob: Job? = null

    private val _stats = MutableStateFlow<StatsTraining>(
        StatsTraining(
            distance = 0.0,
            distanceLap = 0.0,
            elevationGain = 0.0,
            elevationLoss = 0.0,
            rithm = 0.0,
            time = 0.0,
            timeLap = 0.0,
            rithms = listOf(),
            times = listOf()
        )
    )
    val stats = _stats.asStateFlow()

    fun startTimer() {
        if (timerJob?.isActive == true) return

        startTime = SystemClock.elapsedRealtime()

        timerJob = viewModelScope.launch {
            while (true) {
                val now = SystemClock.elapsedRealtime()
                val currentSessionTime = now - startTime
                val currentLapTime = now - lapStartTime

                timeInSeconds.doubleValue = (accumulatedTime + currentSessionTime) / 1000.0
                timeLapInSeconds.doubleValue = (accumulatedTimeLap + currentLapTime) / 1000.0


                delay(100L)
            }
        }
    }

    fun pauseTimer() {
        timerJob?.cancel()

        if (startTime > 0) {
            accumulatedTime += SystemClock.elapsedRealtime() - startTime
            accumulatedTimeLap += SystemClock.elapsedRealtime() - lapStartTime
            startTime = 0
            lapStartTime = 0
        }

    }


    fun registerPoint(lat: Double, lon: Double, alt: Double) {

        val newStats = motorRust.registerNewPoint(lat, lon, alt, timeInSeconds.doubleValue, timeLapInSeconds.doubleValue)

        val currentLapDistance = newStats.distance - lapStartDistance
        val timeLap = SystemClock.elapsedRealtime() - lapStartTime

        if (currentLapDistance >= 1000.0) {
            triggerLap(currentLapDistance, timeLap.toDouble())
        }

        _stats.value = newStats
    }

    fun triggerLap(dist: Double, time: Double) {
        lapStartTime = SystemClock.elapsedRealtime()
        timeLapInSeconds.doubleValue = 0.0
        lapStartDistance = _stats.value.distance

        motorRust.registerLap(time, dist)
    }

    fun endTraining() {
        timerJob?.cancel()

        accumulatedTime = 0
        timeInSeconds.doubleValue = 0.0
        motorRust.endTraining()
    }
}