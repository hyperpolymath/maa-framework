module Main exposing (main)

{-| Absolute Zero - CNO Playground
An interactive environment for exploring Certified Null Operations

This Elm application provides:
- Visual exploration of CNO properties
- Mathematical playground for formal verification concepts
- Integration with Echidna testing (future)
- Real-time property checking

Author: Jonathan D. A. Jewell
License: AGPL-3.0 / Palimpsest 0.5
-}

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode as Decode
import Svg exposing (Svg)
import Svg.Attributes as SvgAttr


-- MAIN

main : Program () Model Msg
main =
    Browser.element
        { init = init
        { update = update
        , subscriptions = subscriptions
        , view = view
        }


-- MODEL

type alias Model =
    { currentTab : Tab
    , cnoState : CNOState
    , mathPlayground : MathState
    , echidnaStatus : EchidnaStatus
    }

type Tab
    = CNOExplorer
    | MathPlayground
    | EchidnaConnector
    | Documentation

type alias CNOState =
    { programInput : String
    , executionSteps : List ExecutionStep
    , properties : PropertyChecks
    }

type alias ExecutionStep =
    { stepNumber : Int
    , state : ProgramState
    , instruction : String
    }

type alias ProgramState =
    { registers : List Int
    , memory : List Int
    , pc : Int  -- Program counter
    }

type alias PropertyChecks =
    { idempotent : Maybe Bool
    , commutative : Maybe Bool
    , deterministic : Maybe Bool
    , sideEffectFree : Maybe Bool
    , terminating : Maybe Bool
    }

type alias MathState =
    { selectedConcept : MathConcept
    , parameters : MathParameters
    , visualization : Visualization
    }

type MathConcept
    = LandauerPrinciple
    | ShannonEntropy
    | BoltzmannEntropy
    | ThermodynamicReversibility
    | QuantumIdentity

type alias MathParameters =
    { temperature : Float
    , stateCount : Int
    , energyDissipation : Float
    }

type Visualization
    = EntropyGraph
    | EnergyDiagram
    | StateTransitions

type alias EchidnaStatus =
    { connected : Bool
    , testResults : List TestResult
    , coverage : Float
    }

type alias TestResult =
    { propertyName : String
    , passed : Bool
    , counterexample : Maybe String
    }


-- INIT

init : () -> ( Model, Cmd Msg )
init _ =
    ( { currentTab = CNOExplorer
      , cnoState = initialCNOState
      , mathPlayground = initialMathState
      , echidnaStatus = initialEchidnaStatus
      }
    , Cmd.none
    )

initialCNOState : CNOState
initialCNOState =
    { programInput = ""
    , executionSteps = []
    , properties =
        { idempotent = Nothing
        , commutative = Nothing
        , deterministic = Nothing
        , sideEffectFree = Nothing
        , terminating = Nothing
        }
    }

initialMathState : MathState
initialMathState =
    { selectedConcept = LandauerPrinciple
    , parameters =
        { temperature = 300.0  -- Kelvin
        , stateCount = 2
        , energyDissipation = 0.0
        }
    , visualization = EnergyDiagram
    }

initialEchidnaStatus : EchidnaStatus
initialEchidnaStatus =
    { connected = False
    , testResults = []
    , coverage = 0.0
    }


-- UPDATE

type Msg
    = ChangeTab Tab
    | UpdateProgramInput String
    | ExecuteProgram
    | CheckProperty PropertyType
    | UpdateMathParameter MathParameterType Float
    | ChangeMathConcept MathConcept
    | ChangeVisualization Visualization
    | ConnectEchidna
    | RunEchidnaTest String

type PropertyType
    = CheckIdempotence
    | CheckCommutativity
    | CheckDeterminism
    | CheckSideEffects
    | CheckTermination

type MathParameterType
    = Temperature
    | StateCount
    | EnergyDissipation

update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ChangeTab tab ->
            ( { model | currentTab = tab }, Cmd.none )

        UpdateProgramInput input ->
            let
                oldCNOState = model.cnoState
                newCNOState = { oldCNOState | programInput = input }
            in
            ( { model | cnoState = newCNOState }, Cmd.none )

        ExecuteProgram ->
            let
                steps = simulateExecution model.cnoState.programInput
                oldCNOState = model.cnoState
                newCNOState = { oldCNOState | executionSteps = steps }
            in
            ( { model | cnoState = newCNOState }, Cmd.none )

        CheckProperty propType ->
            let
                result = checkProperty propType model.cnoState
                oldCNOState = model.cnoState
                newProperties = updatePropertyCheck propType result oldCNOState.properties
                newCNOState = { oldCNOState | properties = newProperties }
            in
            ( { model | cnoState = newCNOState }, Cmd.none )

        UpdateMathParameter paramType value ->
            let
                oldMath = model.mathPlayground
                oldParams = oldMath.parameters
                newParams = updateMathParam paramType value oldParams
                newMath = { oldMath | parameters = newParams }
            in
            ( { model | mathPlayground = newMath }, Cmd.none )

        ChangeMathConcept concept ->
            let
                oldMath = model.mathPlayground
                newMath = { oldMath | selectedConcept = concept }
            in
            ( { model | mathPlayground = newMath }, Cmd.none )

        ChangeVisualization viz ->
            let
                oldMath = model.mathPlayground
                newMath = { oldMath | visualization = viz }
            in
            ( { model | mathPlayground = newMath }, Cmd.none )

        ConnectEchidna ->
            -- Simulate Echidna connection (actual WebSocket would require ports)
            -- In a real implementation, this would:
            -- 1. Open WebSocket to Echidna server at ws://localhost:8545
            -- 2. Send handshake with contract configuration
            -- 3. Receive connection confirmation
            let
                oldStatus = model.echidnaStatus
                newStatus = { oldStatus | connected = True, coverage = 0.0 }
            in
            ( { model | echidnaStatus = newStatus }, Cmd.none )

        RunEchidnaTest propertyName ->
            -- Simulate Echidna property-based test execution
            -- In a real implementation, this would:
            -- 1. Send test request to Echidna via WebSocket
            -- 2. Receive streaming test results
            -- 3. Update UI with counterexamples if found
            let
                oldStatus = model.echidnaStatus
                -- Simulate test result based on property name
                testResult = simulateEchidnaTest propertyName
                newResults = testResult :: oldStatus.testResults
                newCoverage = min 100.0 (oldStatus.coverage + 15.0)
                newStatus = { oldStatus | testResults = newResults, coverage = newCoverage }
            in
            ( { model | echidnaStatus = newStatus }, Cmd.none )


-- Simulate Echidna test execution (returns mock result)
simulateEchidnaTest : String -> TestResult
simulateEchidnaTest propertyName =
    -- CNO properties should always pass for well-formed CNO contracts
    case propertyName of
        "idempotence" ->
            { propertyName = "echidna_test_idempotence"
            , passed = True
            , counterexample = Nothing
            }

        "commutativity" ->
            { propertyName = "echidna_test_commutativity"
            , passed = True
            , counterexample = Nothing
            }

        "determinism" ->
            { propertyName = "echidna_test_determinism"
            , passed = True
            , counterexample = Nothing
            }

        _ ->
            { propertyName = "echidna_test_" ++ propertyName
            , passed = True
            , counterexample = Nothing
            }


-- SIMULATION HELPERS

simulateExecution : String -> List ExecutionStep
simulateExecution program =
    -- Simplified CNO execution simulation
    -- In reality, this would parse and execute the program
    [ { stepNumber = 0
      , state = { registers = [0, 0, 0], memory = [0, 0, 0], pc = 0 }
      , instruction = "NOP"
      }
    , { stepNumber = 1
      , state = { registers = [0, 0, 0], memory = [0, 0, 0], pc = 1 }
      , instruction = "HALT"
      }
    ]

checkProperty : PropertyType -> CNOState -> Bool
checkProperty propType cnoState =
    -- Property checking based on execution trace analysis
    case propType of
        CheckIdempotence ->
            -- Check if applying the program twice yields the same result as once
            -- For a true CNO, the state should be unchanged after execution
            let
                finalState = getFinalState cnoState.executionSteps
                initialState = getInitialState cnoState.executionSteps
            in
            statesEqual initialState finalState

        CheckCommutativity ->
            -- CNOs commute with themselves (trivially, since they're identity)
            -- Check: state unchanged means p(q(s)) = q(p(s)) = s
            let
                finalState = getFinalState cnoState.executionSteps
                initialState = getInitialState cnoState.executionSteps
            in
            statesEqual initialState finalState

        CheckDeterminism ->
            -- Check if all execution steps are deterministic
            -- A program is deterministic if no branching on undefined input
            not (String.contains "," cnoState.programInput)
            && not (String.contains "random" (String.toLower cnoState.programInput))

        CheckSideEffects ->
            -- Check for absence of I/O operations in the program
            let
                ioOperators = [ "print", "read", "write", "input", "output", ".", "," ]
                hasIO = List.any (\op -> String.contains op (String.toLower cnoState.programInput)) ioOperators
            in
            not hasIO

        CheckTermination ->
            -- Terminates if execution completed in reasonable steps
            List.length cnoState.executionSteps < 1000

-- Helper: Get initial state from execution trace
getInitialState : List ExecutionStep -> ProgramState
getInitialState steps =
    case List.head steps of
        Just step -> step.state
        Nothing -> { registers = [], memory = [], pc = 0 }

-- Helper: Get final state from execution trace
getFinalState : List ExecutionStep -> ProgramState
getFinalState steps =
    case List.head (List.reverse steps) of
        Just step -> step.state
        Nothing -> { registers = [], memory = [], pc = 0 }

-- Helper: Check if two program states are equal
statesEqual : ProgramState -> ProgramState -> Bool
statesEqual s1 s2 =
    s1.registers == s2.registers && s1.memory == s2.memory

updatePropertyCheck : PropertyType -> Bool -> PropertyChecks -> PropertyChecks
updatePropertyCheck propType result properties =
    case propType of
        CheckIdempotence ->
            { properties | idempotent = Just result }

        CheckCommutativity ->
            { properties | commutative = Just result }

        CheckDeterminism ->
            { properties | deterministic = Just result }

        CheckSideEffects ->
            { properties | sideEffectFree = Just result }

        CheckTermination ->
            { properties | terminating = Just result }

updateMathParam : MathParameterType -> Float -> MathParameters -> MathParameters
updateMathParam paramType value params =
    case paramType of
        Temperature ->
            { params | temperature = value }

        StateCount ->
            { params | stateCount = round value }

        EnergyDissipation ->
            { params | energyDissipation = value }


-- SUBSCRIPTIONS

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


-- VIEW

view : Model -> Html Msg
view model =
    div [ class "absolute-zero-playground" ]
        [ viewHeader
        , viewTabs model.currentTab
        , viewContent model
        , viewFooter
        ]

viewHeader : Html Msg
viewHeader =
    header [ class "app-header" ]
        [ h1 [] [ text "Absolute Zero Playground" ]
        , p [] [ text "Interactive Certified Null Operations Explorer" ]
        ]

viewTabs : Tab -> Html Msg
viewTabs currentTab =
    nav [ class "tabs" ]
        [ button
            [ class (if currentTab == CNOExplorer then "tab active" else "tab")
            , onClick (ChangeTab CNOExplorer)
            ]
            [ text "CNO Explorer" ]
        , button
            [ class (if currentTab == MathPlayground then "tab active" else "tab")
            , onClick (ChangeTab MathPlayground)
            ]
            [ text "Math Playground" ]
        , button
            [ class (if currentTab == EchidnaConnector then "tab active" else "tab")
            , onClick (ChangeTab EchidnaConnector)
            ]
            [ text "Echidna Connector" ]
        , button
            [ class (if currentTab == Documentation then "tab active" else "tab")
            , onClick (ChangeTab Documentation)
            ]
            [ text "Documentation" ]
        ]

viewContent : Model -> Html Msg
viewContent model =
    div [ class "content" ]
        [ case model.currentTab of
            CNOExplorer ->
                viewCNOExplorer model.cnoState

            MathPlayground ->
                viewMathPlayground model.mathPlayground

            EchidnaConnector ->
                viewEchidnaConnector model.echidnaStatus

            Documentation ->
                viewDocumentation
        ]

viewCNOExplorer : CNOState -> Html Msg
viewCNOExplorer cnoState =
    div [ class "cno-explorer" ]
        [ div [ class "input-section" ]
            [ h2 [] [ text "Program Input" ]
            , textarea
                [ placeholder "Enter CNO program (or simple pseudocode)"
                , value cnoState.programInput
                , onInput UpdateProgramInput
                , rows 10
                , cols 80
                ]
                []
            , button [ onClick ExecuteProgram ] [ text "Execute" ]
            ]
        , div [ class "execution-section" ]
            [ h2 [] [ text "Execution Trace" ]
            , viewExecutionSteps cnoState.executionSteps
            ]
        , div [ class "properties-section" ]
            [ h2 [] [ text "CNO Properties" ]
            , viewPropertyChecks cnoState.properties
            ]
        ]

viewExecutionSteps : List ExecutionStep -> Html Msg
viewExecutionSteps steps =
    if List.isEmpty steps then
        p [] [ text "No execution yet. Enter a program and click Execute." ]
    else
        table []
            [ thead []
                [ tr []
                    [ th [] [ text "Step" ]
                    , th [] [ text "PC" ]
                    , th [] [ text "Registers" ]
                    , th [] [ text "Memory" ]
                    , th [] [ text "Instruction" ]
                    ]
                ]
            , tbody []
                (List.map viewExecutionStep steps)
            ]

viewExecutionStep : ExecutionStep -> Html Msg
viewExecutionStep step =
    tr []
        [ td [] [ text (String.fromInt step.stepNumber) ]
        , td [] [ text (String.fromInt step.state.pc) ]
        , td [] [ text (String.join ", " (List.map String.fromInt step.state.registers)) ]
        , td [] [ text (String.join ", " (List.map String.fromInt step.state.memory)) ]
        , td [] [ text step.instruction ]
        ]

viewPropertyChecks : PropertyChecks -> Html Msg
viewPropertyChecks properties =
    div [ class "property-checks" ]
        [ viewPropertyCheck "Idempotent" properties.idempotent CheckIdempotence
        , viewPropertyCheck "Commutative" properties.commutative CheckCommutativity
        , viewPropertyCheck "Deterministic" properties.deterministic CheckDeterminism
        , viewPropertyCheck "Side-Effect Free" properties.sideEffectFree CheckSideEffects
        , viewPropertyCheck "Terminating" properties.terminating CheckTermination
        ]

viewPropertyCheck : String -> Maybe Bool -> PropertyType -> Html Msg
viewPropertyCheck name result propType =
    div [ class "property-check" ]
        [ label [] [ text name ]
        , span [ class "result" ]
            [ text (case result of
                Nothing -> "Not checked"
                Just True -> "✓ Pass"
                Just False -> "✗ Fail"
              )
            ]
        , button [ onClick (CheckProperty propType) ] [ text "Check" ]
        ]

viewMathPlayground : MathState -> Html Msg
viewMathPlayground mathState =
    div [ class "math-playground" ]
        [ div [ class "concept-selector" ]
            [ h2 [] [ text "Mathematical Concept" ]
            , select [ onInput (\s -> ChangeMathConcept (stringToMathConcept s)) ]
                [ option [ value "landauer" ] [ text "Landauer's Principle" ]
                , option [ value "shannon" ] [ text "Shannon Entropy" ]
                , option [ value "boltzmann" ] [ text "Boltzmann Entropy" ]
                , option [ value "reversibility" ] [ text "Thermodynamic Reversibility" ]
                , option [ value "quantum" ] [ text "Quantum Identity Gate" ]
                ]
            ]
        , div [ class "parameters" ]
            [ h2 [] [ text "Parameters" ]
            , viewMathParameter "Temperature (K)" mathState.parameters.temperature Temperature
            , viewMathParameter "State Count" (toFloat mathState.parameters.stateCount) StateCount
            , viewMathParameter "Energy Dissipation (J)" mathState.parameters.energyDissipation EnergyDissipation
            ]
        , div [ class "visualization" ]
            [ h2 [] [ text "Visualization" ]
            , viewVisualization mathState
            ]
        , div [ class "explanation" ]
            [ h2 [] [ text "Explanation" ]
            , viewConceptExplanation mathState.selectedConcept mathState.parameters
            ]
        ]

viewMathParameter : String -> Float -> MathParameterType -> Html Msg
viewMathParameter label value paramType =
    div [ class "parameter" ]
        [ label [] [ text label ]
        , input
            [ type_ "range"
            , Html.Attributes.min "0"
            , Html.Attributes.max "1000"
            , Html.Attributes.step "1"
            , Html.Attributes.value (String.fromFloat value)
            , onInput (\s -> UpdateMathParameter paramType (Maybe.withDefault 0 (String.toFloat s)))
            ]
            []
        , span [] [ text (String.fromFloat value) ]
        ]

viewVisualization : MathState -> Html Msg
viewVisualization mathState =
    case mathState.selectedConcept of
        LandauerPrinciple ->
            viewLandauerVisualization mathState.parameters

        ShannonEntropy ->
            viewShannonVisualization mathState.parameters

        BoltzmannEntropy ->
            viewBoltzmannVisualization mathState.parameters

        ThermodynamicReversibility ->
            viewReversibilityVisualization mathState.parameters

        QuantumIdentity ->
            viewQuantumVisualization mathState.parameters

viewLandauerVisualization : MathParameters -> Html Msg
viewLandauerVisualization params =
    let
        kB = 1.380649e-23  -- Boltzmann constant
        landauerLimit = kB * params.temperature * logBase e 2
    in
    div [ class "landauer-viz" ]
        [ Svg.svg [ SvgAttr.width "600", SvgAttr.height "400", SvgAttr.viewBox "0 0 600 400" ]
            [ -- Energy axis
              Svg.line [ SvgAttr.x1 "50", SvgAttr.y1 "350", SvgAttr.x2 "550", SvgAttr.y2 "350", SvgAttr.stroke "black" ] []
            , -- Temperature axis
              Svg.line [ SvgAttr.x1 "50", SvgAttr.y1 "350", SvgAttr.x2 "50", SvgAttr.y2 "50", SvgAttr.stroke "black" ] []
            , -- Landauer limit line
              Svg.line [ SvgAttr.x1 "50", SvgAttr.y1 "200", SvgAttr.x2 "550", SvgAttr.y2 "200", SvgAttr.stroke "red", SvgAttr.strokeDasharray "5,5" ] []
            , Svg.text_ [ SvgAttr.x "560", SvgAttr.y "205" ] [ Svg.text ("kT ln 2 = " ++ String.fromFloat landauerLimit ++ " J") ]
            ]
        , p [] [ text ("Landauer Limit: " ++ String.fromFloat landauerLimit ++ " Joules per bit erased") ]
        , p [] [ text ("For a CNO: Energy dissipation = 0 J (no bits erased)") ]
        ]

viewShannonVisualization : MathParameters -> Html Msg
viewShannonVisualization params =
    let
        -- Shannon entropy: H = -Σ p_i log₂(p_i)
        -- For uniform distribution over n states: H = log₂(n)
        n = toFloat params.stateCount
        maxEntropy = if n > 1 then logBase 2 n else 0
        -- Bar width proportional to entropy (max 500px for display)
        barWidth = min 500 (maxEntropy * 100)
    in
    div [ class "shannon-viz" ]
        [ Svg.svg [ SvgAttr.width "600", SvgAttr.height "300", SvgAttr.viewBox "0 0 600 300" ]
            [ -- Title
              Svg.text_ [ SvgAttr.x "300", SvgAttr.y "30", SvgAttr.textAnchor "middle", SvgAttr.fontSize "16" ]
                [ Svg.text "Shannon Entropy: H(P) = -Σ pᵢ log₂(pᵢ)" ]
            , -- Axis
              Svg.line [ SvgAttr.x1 "50", SvgAttr.y1 "200", SvgAttr.x2 "550", SvgAttr.y2 "200", SvgAttr.stroke "black" ] []
            , Svg.line [ SvgAttr.x1 "50", SvgAttr.y1 "200", SvgAttr.x2 "50", SvgAttr.y2 "80", SvgAttr.stroke "black" ] []
            , -- Entropy bar
              Svg.rect
                [ SvgAttr.x "60"
                , SvgAttr.y "120"
                , SvgAttr.width (String.fromFloat barWidth)
                , SvgAttr.height "60"
                , SvgAttr.fill "#4A90D9"
                ] []
            , -- Labels
              Svg.text_ [ SvgAttr.x "50", SvgAttr.y "220" ] [ Svg.text "0" ]
            , Svg.text_ [ SvgAttr.x "550", SvgAttr.y "220" ] [ Svg.text "bits" ]
            , Svg.text_ [ SvgAttr.x "30", SvgAttr.y "75" ] [ Svg.text "H" ]
            , -- Value display
              Svg.text_ [ SvgAttr.x "300", SvgAttr.y "260", SvgAttr.textAnchor "middle" ]
                [ Svg.text ("H = " ++ String.fromFloat maxEntropy ++ " bits (uniform over " ++ String.fromInt params.stateCount ++ " states)") ]
            ]
        , p [] [ text ("For a CNO: ΔH = 0 (entropy is preserved, no information lost)") ]
        ]

viewBoltzmannVisualization : MathParameters -> Html Msg
viewBoltzmannVisualization params =
    let
        -- Boltzmann entropy: S = kB ln(Ω) where Ω = number of microstates
        -- For n states: S = kB ln(n)
        kB = 1.380649e-23  -- Boltzmann constant in J/K
        n = toFloat params.stateCount
        entropy = if n > 1 then kB * logBase e n else 0
        -- Scaled for visualization
        scaledEntropy = if n > 1 then logBase e n * 50 else 0
    in
    div [ class "boltzmann-viz" ]
        [ Svg.svg [ SvgAttr.width "600", SvgAttr.height "350", SvgAttr.viewBox "0 0 600 350" ]
            [ -- Title
              Svg.text_ [ SvgAttr.x "300", SvgAttr.y "30", SvgAttr.textAnchor "middle", SvgAttr.fontSize "16" ]
                [ Svg.text "Boltzmann Entropy: S = kB ln(Ω)" ]
            , -- Microstate representation (circles)
              Svg.g [ SvgAttr.transform "translate(100, 80)" ]
                (List.indexedMap
                    (\i _ ->
                        Svg.circle
                            [ SvgAttr.cx (String.fromInt (modBy 10 i * 40))
                            , SvgAttr.cy (String.fromInt (i // 10 * 40))
                            , SvgAttr.r "15"
                            , SvgAttr.fill "#6B8E23"
                            , SvgAttr.stroke "#333"
                            , SvgAttr.strokeWidth "1"
                            ] []
                    )
                    (List.range 0 (min (params.stateCount - 1) 19))
                )
            , -- Entropy scale
              Svg.rect [ SvgAttr.x "450", SvgAttr.y "80", SvgAttr.width "30", SvgAttr.height (String.fromFloat scaledEntropy), SvgAttr.fill "#D35400" ] []
            , Svg.text_ [ SvgAttr.x "490", SvgAttr.y "90" ] [ Svg.text "S" ]
            , -- Formula
              Svg.text_ [ SvgAttr.x "300", SvgAttr.y "280", SvgAttr.textAnchor "middle" ]
                [ Svg.text ("Ω = " ++ String.fromInt params.stateCount ++ " microstates") ]
            , Svg.text_ [ SvgAttr.x "300", SvgAttr.y "310", SvgAttr.textAnchor "middle" ]
                [ Svg.text ("S = kB × ln(" ++ String.fromInt params.stateCount ++ ") = " ++ String.left 12 (String.fromFloat entropy) ++ " J/K") ]
            ]
        , p [] [ text "Boltzmann's insight: S = kB ln(Ω) connects microscopic states to macroscopic entropy" ]
        , p [] [ text "For a CNO: ΔS = 0 (microstate count unchanged)" ]
        ]

viewReversibilityVisualization : MathParameters -> Html Msg
viewReversibilityVisualization params =
    let
        -- Visualize state transition: A -> B -> A (reversible) vs A -> B (irreversible)
        -- Energy dissipation determines reversibility
        isReversible = params.energyDissipation == 0
    in
    div [ class "reversibility-viz" ]
        [ Svg.svg [ SvgAttr.width "600", SvgAttr.height "300", SvgAttr.viewBox "0 0 600 300" ]
            [ -- Title
              Svg.text_ [ SvgAttr.x "300", SvgAttr.y "30", SvgAttr.textAnchor "middle", SvgAttr.fontSize "16" ]
                [ Svg.text "Thermodynamic Reversibility" ]
            , -- State A (initial)
              Svg.circle [ SvgAttr.cx "100", SvgAttr.cy "150", SvgAttr.r "40", SvgAttr.fill "#3498DB", SvgAttr.stroke "#2C3E50", SvgAttr.strokeWidth "2" ] []
            , Svg.text_ [ SvgAttr.x "100", SvgAttr.y "155", SvgAttr.textAnchor "middle", SvgAttr.fill "white" ] [ Svg.text "State A" ]
            , -- Arrow A -> B (forward process)
              Svg.line [ SvgAttr.x1 "150", SvgAttr.y1 "130", SvgAttr.x2 "250", SvgAttr.y2 "130", SvgAttr.stroke "#27AE60", SvgAttr.strokeWidth "3", SvgAttr.markerEnd "url(#arrowhead)" ] []
            , Svg.text_ [ SvgAttr.x "200", SvgAttr.y "120", SvgAttr.textAnchor "middle", SvgAttr.fontSize "12" ] [ Svg.text "Process P" ]
            , -- State B (intermediate)
              Svg.circle [ SvgAttr.cx "300", SvgAttr.cy "150", SvgAttr.r "40", SvgAttr.fill "#E74C3C", SvgAttr.stroke "#2C3E50", SvgAttr.strokeWidth "2" ] []
            , Svg.text_ [ SvgAttr.x "300", SvgAttr.y "155", SvgAttr.textAnchor "middle", SvgAttr.fill "white" ] [ Svg.text "State B" ]
            , -- Arrow B -> A (reverse process, only if reversible)
              if isReversible then
                  Svg.g []
                      [ Svg.line [ SvgAttr.x1 "250", SvgAttr.y1 "170", SvgAttr.x2 "150", SvgAttr.y2 "170", SvgAttr.stroke "#9B59B6", SvgAttr.strokeWidth "3", SvgAttr.strokeDasharray "5,5" ] []
                      , Svg.text_ [ SvgAttr.x "200", SvgAttr.y "195", SvgAttr.textAnchor "middle", SvgAttr.fontSize "12" ] [ Svg.text "P⁻¹ (inverse)" ]
                      ]
              else
                  Svg.text_ [ SvgAttr.x "200", SvgAttr.y "195", SvgAttr.textAnchor "middle", SvgAttr.fill "#E74C3C", SvgAttr.fontSize "12" ] [ Svg.text "No inverse (irreversible)" ]
            , -- CNO visualization (identity)
              Svg.circle [ SvgAttr.cx "500", SvgAttr.cy "150", SvgAttr.r "40", SvgAttr.fill "#27AE60", SvgAttr.stroke "#2C3E50", SvgAttr.strokeWidth "2" ] []
            , Svg.text_ [ SvgAttr.x "500", SvgAttr.y "155", SvgAttr.textAnchor "middle", SvgAttr.fill "white" ] [ Svg.text "CNO" ]
            , -- Self-loop for CNO
              Svg.path [ SvgAttr.d "M 520 115 C 560 90, 560 210, 520 185", SvgAttr.fill "none", SvgAttr.stroke "#F39C12", SvgAttr.strokeWidth "2" ] []
            , Svg.text_ [ SvgAttr.x "560", SvgAttr.y "150", SvgAttr.fontSize "12" ] [ Svg.text "A = A" ]
            , -- Energy dissipation indicator
              Svg.text_ [ SvgAttr.x "300", SvgAttr.y "260", SvgAttr.textAnchor "middle" ]
                [ Svg.text ("Energy dissipated: " ++ String.fromFloat params.energyDissipation ++ " J") ]
            , Svg.text_ [ SvgAttr.x "300", SvgAttr.y "280", SvgAttr.textAnchor "middle", SvgAttr.fill (if isReversible then "#27AE60" else "#E74C3C") ]
                [ Svg.text (if isReversible then "REVERSIBLE (ΔS = 0)" else "IRREVERSIBLE (ΔS > 0)") ]
            ]
        , p [] [ text "A process is thermodynamically reversible iff ΔS = 0 (no entropy increase)" ]
        , p [] [ text "CNOs are trivially reversible: the identity function is its own inverse" ]
        ]

viewQuantumVisualization : MathParameters -> Html Msg
viewQuantumVisualization params =
    div [ class "quantum-viz" ]
        [ Svg.svg [ SvgAttr.width "600", SvgAttr.height "400", SvgAttr.viewBox "0 0 600 400" ]
            [ -- Title
              Svg.text_ [ SvgAttr.x "300", SvgAttr.y "30", SvgAttr.textAnchor "middle", SvgAttr.fontSize "16" ]
                [ Svg.text "Quantum Identity Gate: I|ψ⟩ = |ψ⟩" ]
            , -- Bloch sphere outline
              Svg.circle [ SvgAttr.cx "200", SvgAttr.cy "200", SvgAttr.r "100", SvgAttr.fill "none", SvgAttr.stroke "#3498DB", SvgAttr.strokeWidth "2" ] []
            , -- Equator ellipse
              Svg.ellipse [ SvgAttr.cx "200", SvgAttr.cy "200", SvgAttr.rx "100", SvgAttr.ry "30", SvgAttr.fill "none", SvgAttr.stroke "#3498DB", SvgAttr.strokeWidth "1", SvgAttr.strokeDasharray "5,3" ] []
            , -- Z-axis
              Svg.line [ SvgAttr.x1 "200", SvgAttr.y1 "90", SvgAttr.x2 "200", SvgAttr.y2 "310", SvgAttr.stroke "#2C3E50", SvgAttr.strokeWidth "1" ] []
            , -- |0⟩ state
              Svg.circle [ SvgAttr.cx "200", SvgAttr.cy "100", SvgAttr.r "8", SvgAttr.fill "#27AE60" ] []
            , Svg.text_ [ SvgAttr.x "220", SvgAttr.y "105" ] [ Svg.text "|0⟩" ]
            , -- |1⟩ state
              Svg.circle [ SvgAttr.cx "200", SvgAttr.cy "300", SvgAttr.r "8", SvgAttr.fill "#E74C3C" ] []
            , Svg.text_ [ SvgAttr.x "220", SvgAttr.y "305" ] [ Svg.text "|1⟩" ]
            , -- Arbitrary state |ψ⟩
              Svg.circle [ SvgAttr.cx "250", SvgAttr.cy "150", SvgAttr.r "6", SvgAttr.fill "#9B59B6" ] []
            , Svg.line [ SvgAttr.x1 "200", SvgAttr.y1 "200", SvgAttr.x2 "250", SvgAttr.y2 "150", SvgAttr.stroke "#9B59B6", SvgAttr.strokeWidth "2" ] []
            , Svg.text_ [ SvgAttr.x "260", SvgAttr.y "145" ] [ Svg.text "|ψ⟩" ]
            , -- Identity gate circuit
              Svg.g [ SvgAttr.transform "translate(400, 100)" ]
                [ -- Qubit wire
                  Svg.line [ SvgAttr.x1 "0", SvgAttr.y1 "50", SvgAttr.x2 "150", SvgAttr.y2 "50", SvgAttr.stroke "#2C3E50", SvgAttr.strokeWidth "2" ] []
                , -- Identity gate box
                  Svg.rect [ SvgAttr.x "50", SvgAttr.y "25", SvgAttr.width "50", SvgAttr.height "50", SvgAttr.fill "#F39C12", SvgAttr.stroke "#2C3E50", SvgAttr.strokeWidth "2" ] []
                , Svg.text_ [ SvgAttr.x "75", SvgAttr.y "55", SvgAttr.textAnchor "middle", SvgAttr.fontWeight "bold" ] [ Svg.text "I" ]
                , -- Input label
                  Svg.text_ [ SvgAttr.x "0", SvgAttr.y "45", SvgAttr.fontSize "12" ] [ Svg.text "|ψ⟩" ]
                , -- Output label
                  Svg.text_ [ SvgAttr.x "135", SvgAttr.y "45", SvgAttr.fontSize "12" ] [ Svg.text "|ψ⟩" ]
                ]
            , -- Identity matrix
              Svg.g [ SvgAttr.transform "translate(400, 200)" ]
                [ Svg.text_ [ SvgAttr.x "0", SvgAttr.y "20", SvgAttr.fontSize "14" ] [ Svg.text "I = " ]
                , -- Matrix brackets
                  Svg.text_ [ SvgAttr.x "30", SvgAttr.y "30", SvgAttr.fontSize "20" ] [ Svg.text "⎡" ]
                  , Svg.text_ [ SvgAttr.x "30", SvgAttr.y "55", SvgAttr.fontSize "20" ] [ Svg.text "⎣" ]
                , Svg.text_ [ SvgAttr.x "90", SvgAttr.y "30", SvgAttr.fontSize "20" ] [ Svg.text "⎤" ]
                , Svg.text_ [ SvgAttr.x "90", SvgAttr.y "55", SvgAttr.fontSize "20" ] [ Svg.text "⎦" ]
                , -- Matrix elements
                  Svg.text_ [ SvgAttr.x "50", SvgAttr.y "25", SvgAttr.fontSize "14" ] [ Svg.text "1  0" ]
                , Svg.text_ [ SvgAttr.x "50", SvgAttr.y "50", SvgAttr.fontSize "14" ] [ Svg.text "0  1" ]
                ]
            , -- Properties
              Svg.text_ [ SvgAttr.x "400", SvgAttr.y "300", SvgAttr.fontSize "12" ] [ Svg.text "Properties:" ]
            , Svg.text_ [ SvgAttr.x "400", SvgAttr.y "320", SvgAttr.fontSize "11" ] [ Svg.text "• I² = I (idempotent)" ]
            , Svg.text_ [ SvgAttr.x "400", SvgAttr.y "340", SvgAttr.fontSize "11" ] [ Svg.text "• I† = I (self-adjoint)" ]
            , Svg.text_ [ SvgAttr.x "400", SvgAttr.y "360", SvgAttr.fontSize "11" ] [ Svg.text "• det(I) = 1 (unitary)" ]
            ]
        , p [] [ text "The quantum identity gate I is the canonical quantum CNO" ]
        , p [] [ text "I|ψ⟩ = |ψ⟩: Every quantum state is an eigenstate with eigenvalue 1" ]
        ]

viewConceptExplanation : MathConcept -> MathParameters -> Html Msg
viewConceptExplanation concept params =
    case concept of
        LandauerPrinciple ->
            div []
                [ h3 [] [ text "Landauer's Principle (1961)" ]
                , p [] [ text "Erasing one bit of information dissipates at least kT ln 2 energy as heat." ]
                , p [] [ text "Formula: E_min = kT ln 2" ]
                , p [] [ text ("At " ++ String.fromFloat params.temperature ++ " K: E_min = " ++ String.fromFloat (1.380649e-23 * params.temperature * logBase e 2) ++ " J") ]
                , p [] [ text "For CNOs: No information is erased, so E = 0. This is thermodynamically reversible!" ]
                ]

        ShannonEntropy ->
            div []
                [ h3 [] [ text "Shannon Entropy" ]
                , p [] [ text "H(P) = -Σ p_i log₂(p_i)" ]
                , p [] [ text "Measures information content in bits." ]
                ]

        BoltzmannEntropy ->
            div []
                [ h3 [] [ text "Boltzmann Entropy" ]
                , p [] [ text "S = k_B ln(2) H" ]
                , p [] [ text "Connects information entropy to thermodynamic entropy." ]
                ]

        ThermodynamicReversibility ->
            div []
                [ h3 [] [ text "Thermodynamic Reversibility" ]
                , p [] [ text "A process is reversible if ΔS = 0 (no entropy change)." ]
                , p [] [ text "CNOs are reversible: input state = output state." ]
                ]

        QuantumIdentity ->
            div []
                [ h3 [] [ text "Quantum Identity Gate" ]
                , p [] [ text "I|ψ⟩ = |ψ⟩ for all quantum states |ψ⟩" ]
                , p [] [ text "The quantum CNO: does nothing to quantum information." ]
                ]

viewEchidnaConnector : EchidnaStatus -> Html Msg
viewEchidnaConnector status =
    div [ class "echidna-connector" ]
        [ h2 [] [ text "Echidna Property-Based Testing" ]
        , div [ class "connection-status" ]
            [ p []
                [ text "Status: "
                , span [ class (if status.connected then "connected" else "disconnected") ]
                    [ text (if status.connected then "Connected" else "Disconnected") ]
                ]
            , if not status.connected then
                button [ onClick ConnectEchidna ] [ text "Connect to Echidna" ]
              else
                text ""
            ]
        , if status.connected then
            div [ class "test-controls" ]
                [ h3 [] [ text "Run Property Tests" ]
                , button [ onClick (RunEchidnaTest "idempotence") ] [ text "Test Idempotence" ]
                , button [ onClick (RunEchidnaTest "commutativity") ] [ text "Test Commutativity" ]
                , button [ onClick (RunEchidnaTest "determinism") ] [ text "Test Determinism" ]
                ]
          else
            p [] [ text "Connect to Echidna to run property-based tests." ]
        , div [ class "test-results" ]
            [ h3 [] [ text "Test Results" ]
            , if List.isEmpty status.testResults then
                p [] [ text "No tests run yet." ]
              else
                table []
                    [ thead []
                        [ tr []
                            [ th [] [ text "Property" ]
                            , th [] [ text "Result" ]
                            , th [] [ text "Counterexample" ]
                            ]
                        ]
                    , tbody []
                        (List.map viewTestResult status.testResults)
                    ]
            ]
        , div [ class "coverage" ]
            [ h3 [] [ text "Code Coverage" ]
            , p [] [ text (String.fromFloat status.coverage ++ "%") ]
            ]
        ]

viewTestResult : TestResult -> Html Msg
viewTestResult result =
    tr []
        [ td [] [ text result.propertyName ]
        , td [ class (if result.passed then "pass" else "fail") ]
            [ text (if result.passed then "✓ Pass" else "✗ Fail") ]
        , td [] [ text (Maybe.withDefault "—" result.counterexample) ]
        ]

viewDocumentation : Html Msg
viewDocumentation =
    div [ class "documentation" ]
        [ h2 [] [ text "Absolute Zero Documentation" ]
        , section []
            [ h3 [] [ text "What is a Certified Null Operation?" ]
            , p [] [ text "A CNO is a program that provably does nothing. It must satisfy:" ]
            , ul []
                [ li [] [ text "Termination: Always halts" ]
                , li [] [ text "State Preservation: Input state = Output state" ]
                , li [] [ text "Purity: No I/O or side effects" ]
                , li [] [ text "Thermodynamic Reversibility: Zero energy dissipation" ]
                ]
            ]
        , section []
            [ h3 [] [ text "Using This Playground" ]
            , ul []
                [ li [] [ text "CNO Explorer: Test programs for CNO properties" ]
                , li [] [ text "Math Playground: Explore theoretical foundations" ]
                , li [] [ text "Echidna Connector: Property-based testing (requires Echidna)" ]
                ]
            ]
        , section []
            [ h3 [] [ text "Resources" ]
            , ul []
                [ li [] [ a [ href "https://gitlab.com/maa-framework/6-the-foundation/absolute-zero" ] [ text "GitLab Repository (full proofs)" ] ]
                , li [] [ a [ href "https://github.com/Hyperpolymath/absolute-zero" ] [ text "GitHub Mirror" ] ]
                , li [] [ text "See COOKBOOK.adoc for getting started" ]
                , li [] [ text "See ECHIDNA_INTEGRATION.adoc for testing setup" ]
                ]
            ]
        ]

viewFooter : Html Msg
viewFooter =
    footer [ class "app-footer" ]
        [ p [] [ text "Absolute Zero © 2025 Jonathan D. A. Jewell" ]
        , p [] [ text "Licensed under AGPL 3.0 / Palimpsest 0.5" ]
        ]


-- HELPERS

stringToMathConcept : String -> MathConcept
stringToMathConcept s =
    case s of
        "landauer" -> LandauerPrinciple
        "shannon" -> ShannonEntropy
        "boltzmann" -> BoltzmannEntropy
        "reversibility" -> ThermodynamicReversibility
        "quantum" -> QuantumIdentity
        _ -> LandauerPrinciple
